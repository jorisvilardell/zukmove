use futures::future::join_all;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::gateway::AggregatedOffer;
use crate::domain::entities::internship::{CreateInternshipRequest, Internship, InternshipStatus};
use crate::domain::ports::{
    DomainError, IntelligenceClient, InternshipRepository, OfferClient, StudentRepository,
};

/// Service d'orchestration pour l'inscription aux stages et passerelle d'API.
pub struct InternshipService {
    student_repo: Box<dyn StudentRepository>,
    internship_repo: Box<dyn InternshipRepository>,
    offer_client: Box<dyn OfferClient>,
    intelligence_client: Arc<dyn IntelligenceClient>,
}

impl InternshipService {
    pub fn new(
        student_repo: Box<dyn StudentRepository>,
        internship_repo: Box<dyn InternshipRepository>,
        offer_client: Box<dyn OfferClient>,
        intelligence_client: Arc<dyn IntelligenceClient>,
    ) -> Self {
        Self {
            student_repo,
            internship_repo,
            offer_client,
            intelligence_client,
        }
    }

    /// Processus d'inscription :
    /// 1. Récupérer l'offre via le client HTTP (Erasmumu)
    /// 2. Récupérer l'étudiant depuis la BDD locale
    /// 3. Valider la correspondance des domaines
    /// 4. Sauvegarder et retourner le résultat
    pub async fn register(
        &self,
        request: CreateInternshipRequest,
    ) -> Result<Internship, DomainError> {
        // 1. Récupérer l'offre depuis Erasmumu
        let offer = self.offer_client.get_offer_by_id(request.offer_id).await?;

        // 2. Récupérer l'étudiant
        let student = self.student_repo.find_by_id(request.student_id).await?;

        // 3. Valider la correspondance des domaines
        let (status, message) = if offer.domain.to_lowercase() == student.domain.to_lowercase() {
            (
                InternshipStatus::Approved,
                format!(
                    "Inscription approuvée : {} {} pour l'offre '{}'",
                    student.firstname, student.name, offer.title
                ),
            )
        } else {
            (
                InternshipStatus::Rejected,
                format!(
                    "Inscription rejetée : le domaine de l'étudiant ({}) ne correspond pas au domaine de l'offre ({})",
                    student.domain, offer.domain
                ),
            )
        };

        // 4. Sauvegarder le résultat
        let internship = Internship {
            id: Uuid::new_v4(),
            student_id: request.student_id,
            offer_id: request.offer_id,
            status,
            message,
        };

        self.internship_repo.save(&internship).await
    }

    /// Récupère une demande d'inscription par son ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Internship, DomainError> {
        self.internship_repo.find_by_id(id).await
    }

    /// Passerelle d'API : Agrégation d'offres et de renseignements (Erasmumu + MI8)
    pub async fn get_aggregated_offers(
        &self,
        domain: Option<String>,
        city: Option<String>,
    ) -> Result<Vec<AggregatedOffer>, DomainError> {
        // 1. Fetch offers from Erasmumu
        let offers = self.offer_client.search_offers(domain, city).await?;

        // Optionnel : Limiter pour ne pas exploser la mémoire ?
        // offers.truncate(limit.unwrap_or(20) as usize);

        // 2. Identify unique cities
        let unique_cities: HashSet<String> = offers.iter().map(|o| o.city.clone()).collect();

        // 3. Fetch intelligence data concurrently for all unique cities
        let mut city_futures = Vec::new();
        for city_name in unique_cities {
            let intelligence = self.intelligence_client.clone();
            let cname = city_name.clone();

            city_futures.push(async move {
                // Fetch score and news concurrently for a specific city
                let (score_res, news_res) = tokio::join!(
                    intelligence.get_city_score(&cname),
                    intelligence.get_latest_news_in_city(&cname, 3)
                );

                let score = score_res.unwrap_or(None);
                let news = news_res.unwrap_or_else(|_| vec![]);

                (cname, score, news)
            });
        }

        let city_results = join_all(city_futures).await;

        let mut intelligence_map = HashMap::new();
        for (cname, score, news) in city_results {
            intelligence_map.insert(cname.clone(), (score, news));
        }

        // 4. Aggregate data
        let aggregated = offers
            .into_iter()
            .map(|offer| {
                let (city_score, latest_news) = intelligence_map
                    .get(&offer.city)
                    .cloned()
                    .unwrap_or((None, vec![]));

                AggregatedOffer {
                    offer,
                    city_score,
                    latest_news: if latest_news.is_empty() {
                        None
                    } else {
                        Some(latest_news)
                    },
                }
            })
            .collect();

        Ok(aggregated)
    }

    /// Récupère les offres recommandées pour un étudiant (filtrées par domaine, triées par score global de la ville)
    pub async fn get_recommended_offers_for_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<AggregatedOffer>, DomainError> {
        let student = self.student_repo.find_by_id(student_id).await?;

        let mut aggregated = self
            .get_aggregated_offers(Some(student.domain), None)
            .await?;

        // Tri descendant par le total du score de la ville
        aggregated.sort_by(|a, b| {
            let score_a = a
                .city_score
                .as_ref()
                .map(|s| s.quality_of_life + s.safety + s.economy + s.culture)
                .unwrap_or(0);
            let score_b = b
                .city_score
                .as_ref()
                .map(|s| s.quality_of_life + s.safety + s.economy + s.culture)
                .unwrap_or(0);

            score_b.cmp(&score_a)
        });

        Ok(aggregated)
    }
}
