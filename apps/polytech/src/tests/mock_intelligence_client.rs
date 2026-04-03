use async_trait::async_trait;
use zukmove_core::domain::entities::city_score::CityScore;
use zukmove_core::domain::entities::news::News;
use zukmove_core::domain::ports::{DomainError, IntelligenceClient};

#[derive(Clone)]
pub struct MockIntelligenceClient {}

impl MockIntelligenceClient {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IntelligenceClient for MockIntelligenceClient {
    async fn get_latest_news(&self, _limit: i32) -> Result<Vec<News>, DomainError> {
        Ok(vec![])
    }

    async fn get_latest_news_in_city(
        &self,
        _city: &str,
        _limit: i32,
    ) -> Result<Vec<News>, DomainError> {
        Ok(vec![])
    }

    async fn get_city_score(&self, city: &str) -> Result<Option<CityScore>, DomainError> {
        // Return a default score for tests
        Ok(Some(CityScore::new(city, "France")))
    }
}
