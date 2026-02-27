use async_trait::async_trait;
use redis::AsyncCommands;
use zukmove_core::domain::entities::news::News;
use zukmove_core::domain::ports::{DomainError, NewsRepository};

/// Redis-backed NewsRepository using Sorted Sets.
///
/// Data structure:
/// - `news:all` (Sorted Set): all news, score = timestamp
/// - `news:city:<city>` (Sorted Set): news filtered by city, score = timestamp
/// - `news:data:<id>` (String): JSON serialized News object
pub struct RedisNewsRepository {
    client: redis::Client,
}

impl RedisNewsRepository {
    pub fn new(redis_url: &str) -> Result<Self, DomainError> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(Self { client })
    }

    async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection, DomainError> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    fn date_to_score(date: &str) -> f64 {
        // Parse YYYY-MM-DD to a numeric score (days since epoch-like)
        chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() as f64)
            .unwrap_or(0.0)
    }
}

#[async_trait]
impl NewsRepository for RedisNewsRepository {
    async fn save(&self, news: &News) -> Result<News, DomainError> {
        let mut conn = self.get_connection().await?;

        let json = serde_json::to_string(news)
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        let score = Self::date_to_score(&news.date);
        let city_key = format!("news:city:{}", news.city.to_lowercase());
        let data_key = format!("news:data:{}", news.id);

        // Store the JSON data
        let _: () = conn
            .set(&data_key, &json)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        // Add to global sorted set
        let _: () = conn
            .zadd("news:all", &news.id, score)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        // Add to city sorted set
        let _: () = conn
            .zadd(&city_key, &news.id, score)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(news.clone())
    }

    async fn get_latest(&self, limit: usize) -> Result<Vec<News>, DomainError> {
        let mut conn = self.get_connection().await?;

        // Get the latest N IDs from the sorted set (reverse order = newest first)
        let ids: Vec<String> = conn
            .zrevrange("news:all", 0, (limit as isize) - 1)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        let mut news_list = Vec::new();
        for id in ids {
            let data_key = format!("news:data:{}", id);
            let json: Option<String> = conn
                .get(&data_key)
                .await
                .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

            if let Some(json) = json {
                let news: News = serde_json::from_str(&json)
                    .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
                news_list.push(news);
            }
        }

        Ok(news_list)
    }

    async fn get_latest_in_city(&self, city: &str, limit: usize) -> Result<Vec<News>, DomainError> {
        let mut conn = self.get_connection().await?;

        let city_key = format!("news:city:{}", city.to_lowercase());

        let ids: Vec<String> = conn
            .zrevrange(&city_key, 0, (limit as isize) - 1)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        let mut news_list = Vec::new();
        for id in ids {
            let data_key = format!("news:data:{}", id);
            let json: Option<String> = conn
                .get(&data_key)
                .await
                .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

            if let Some(json) = json {
                let news: News = serde_json::from_str(&json)
                    .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
                news_list.push(news);
            }
        }

        Ok(news_list)
    }
}
