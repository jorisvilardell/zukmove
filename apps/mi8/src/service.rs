use tonic::{Request, Response, Status};

use crate::proto::mi8_service_server::Mi8Service;
use crate::proto::{
    CityScore as ProtoCityScore, CityScoreList, CreateNewsRequest, GetCityScoreRequest,
    GetLatestNewsInCityRequest, GetLatestNewsRequest, GetTopCitiesRequest, News as ProtoNews,
    NewsList,
};
use zukmove_core::domain::entities::news::News;
use zukmove_core::domain::ports::{CityScoreRepository, NewsRepository};

pub struct Mi8ServiceImpl {
    news_repo: Box<dyn NewsRepository>,
    city_score_repo: Box<dyn CityScoreRepository>,
}

impl Mi8ServiceImpl {
    pub fn new(
        news_repo: Box<dyn NewsRepository>,
        city_score_repo: Box<dyn CityScoreRepository>,
    ) -> Self {
        Self {
            news_repo,
            city_score_repo,
        }
    }
}

// ─── Conversion helpers ───

fn domain_news_to_proto(news: &News) -> ProtoNews {
    ProtoNews {
        id: news.id.clone(),
        name: news.name.clone(),
        source: news.source.clone(),
        date: news.date.clone(),
        tags: news.tags.clone(),
        city: news.city.clone(),
        country: news.country.clone(),
    }
}

fn domain_score_to_proto(
    score: &zukmove_core::domain::entities::city_score::CityScore,
) -> ProtoCityScore {
    ProtoCityScore {
        city: score.city.clone(),
        country: score.country.clone(),
        updated_at: score.updated_at.clone(),
        quality_of_life: score.quality_of_life,
        safety: score.safety,
        economy: score.economy,
        culture: score.culture,
    }
}

#[tonic::async_trait]
impl Mi8Service for Mi8ServiceImpl {
    async fn get_latest_news(
        &self,
        request: Request<GetLatestNewsRequest>,
    ) -> Result<Response<NewsList>, Status> {
        let limit = request.into_inner().limit as usize;
        let limit = if limit == 0 { 10 } else { limit };

        let news = self
            .news_repo
            .get_latest(limit)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(NewsList {
            news: news.iter().map(domain_news_to_proto).collect(),
        }))
    }

    async fn get_latest_news_in_city(
        &self,
        request: Request<GetLatestNewsInCityRequest>,
    ) -> Result<Response<NewsList>, Status> {
        let req = request.into_inner();
        let limit = if req.limit == 0 { 10 } else { req.limit as usize };

        if req.city.is_empty() {
            return Err(Status::invalid_argument("city is required"));
        }

        let news = self
            .news_repo
            .get_latest_in_city(&req.city, limit)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(NewsList {
            news: news.iter().map(domain_news_to_proto).collect(),
        }))
    }

    async fn create_news(
        &self,
        request: Request<CreateNewsRequest>,
    ) -> Result<Response<ProtoNews>, Status> {
        let req = request.into_inner();

        if req.name.is_empty() || req.city.is_empty() {
            return Err(Status::invalid_argument("name and city are required"));
        }

        let news = News {
            id: uuid::Uuid::new_v4().to_string(),
            name: req.name,
            source: req.source,
            date: if req.date.is_empty() {
                chrono::Utc::now().format("%Y-%m-%d").to_string()
            } else {
                req.date
            },
            tags: req.tags.clone(),
            city: req.city.clone(),
            country: req.country.clone(),
        };

        // Save the news
        let saved = self
            .news_repo
            .save(&news)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Update city score based on tags
        if !req.tags.is_empty() {
            let mut score = self
                .city_score_repo
                .get_or_create(&req.city, &req.country)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            score.apply_tags(&req.tags);

            self.city_score_repo
                .save(&score)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
        }

        Ok(Response::new(domain_news_to_proto(&saved)))
    }

    async fn get_city_score(
        &self,
        request: Request<GetCityScoreRequest>,
    ) -> Result<Response<ProtoCityScore>, Status> {
        let city = request.into_inner().city;

        if city.is_empty() {
            return Err(Status::invalid_argument("city is required"));
        }

        let score = self
            .city_score_repo
            .get_or_create(&city, "")
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(domain_score_to_proto(&score)))
    }

    async fn get_top_cities(
        &self,
        request: Request<GetTopCitiesRequest>,
    ) -> Result<Response<CityScoreList>, Status> {
        let limit = request.into_inner().limit as usize;
        let limit = if limit == 0 { 10 } else { limit };

        let scores = self
            .city_score_repo
            .get_top_cities(limit)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CityScoreList {
            scores: scores.iter().map(domain_score_to_proto).collect(),
        }))
    }
}
