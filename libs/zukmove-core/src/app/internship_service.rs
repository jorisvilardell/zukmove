use uuid::Uuid;

use crate::domain::entities::internship::{CreateInternshipRequest, Internship, InternshipStatus};
use crate::domain::ports::{DomainError, InternshipRepository, OfferClient, StudentRepository};

/// Service d'orchestration pour l'inscription aux stages.
pub struct InternshipService {
    student_repo: Box<dyn StudentRepository>,
    internship_repo: Box<dyn InternshipRepository>,
    offer_client: Box<dyn OfferClient>,
}

impl InternshipService {
    pub fn new(
        student_repo: Box<dyn StudentRepository>,
        internship_repo: Box<dyn InternshipRepository>,
        offer_client: Box<dyn OfferClient>,
    ) -> Self {
        Self {
            student_repo,
            internship_repo,
            offer_client,
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
}
