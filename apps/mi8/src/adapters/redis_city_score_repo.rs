use async_trait::async_trait;
use redis::AsyncCommands;
use zukmove_core::domain::entities::city_score::CityScore;
use zukmove_core::domain::ports::{CityScoreRepository, DomainError};

/// Redis-backed CityScoreRepository.
///
/// Data structure:
/// - `cityscore:data:<city>` (String): JSON serialized CityScore
/// - `cityscore:ranking` (Sorted Set): cities ranked by total score
pub struct RedisCityScoreRepository {
    client: redis::Client,
}

impl RedisCityScoreRepository {
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
}

#[async_trait]
impl CityScoreRepository for RedisCityScoreRepository {
    async fn get_or_create(&self, city: &str, country: &str) -> Result<CityScore, DomainError> {
        let mut conn = self.get_connection().await?;
        let data_key = format!("cityscore:data:{}", city.to_lowercase());

        let json: Option<String> = conn
            .get(&data_key)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        if let Some(json) = json {
            let score: CityScore = serde_json::from_str(&json)
                .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
            Ok(score)
        } else {
            let score = CityScore::new(city, country);
            // Save immediately
            self.save(&score).await?;
            Ok(score)
        }
    }

    async fn save(&self, score: &CityScore) -> Result<CityScore, DomainError> {
        let mut conn = self.get_connection().await?;
        let data_key = format!("cityscore:data:{}", score.city.to_lowercase());

        let json = serde_json::to_string(score)
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        // Store JSON data
        let _: () = conn
            .set(&data_key, &json)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        // Update ranking sorted set (ascending: lower score first)
        let _: () = conn
            .zadd(
                "cityscore:ranking",
                &score.city.to_lowercase(),
                score.total_score(),
            )
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(score.clone())
    }

    async fn get_top_cities(&self, limit: usize) -> Result<Vec<CityScore>, DomainError> {
        let mut conn = self.get_connection().await?;

        // Ascending order as per requirement
        let keys: Vec<String> = conn
            .zrange("cityscore:ranking", 0, (limit as isize) - 1)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        let mut scores = Vec::new();
        for key in keys {
            let data_key = format!("cityscore:data:{}", key);
            let json: Option<String> = conn
                .get(&data_key)
                .await
                .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

            if let Some(json) = json {
                let score: CityScore = serde_json::from_str(&json)
                    .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
                scores.push(score);
            }
        }

        Ok(scores)
    }
}
