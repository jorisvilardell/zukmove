use async_trait::async_trait;
use tonic::transport::Channel;
use zukmove_core::domain::ports::{DomainError, IntelligenceClient};

use crate::proto::mi8_service_client::Mi8ServiceClient;
use crate::proto::{GetLatestNewsInCityRequest, GetLatestNewsRequest};

/// Client gRPC pour appeler le service MI8.
#[derive(Clone)]
pub struct GrpcNewsClient {
    client: Mi8ServiceClient<Channel>,
}

impl GrpcNewsClient {
    pub async fn new(mi8_url: String) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let client = Mi8ServiceClient::connect(mi8_url).await?;
        Ok(Self { client })
    }
}

#[async_trait]
impl IntelligenceClient for GrpcNewsClient {
    async fn get_latest_news(
        &self,
        limit: i32,
    ) -> Result<Vec<zukmove_core::domain::entities::news::News>, DomainError> {
        let mut client = self.client.clone();
        let request = tonic::Request::new(GetLatestNewsRequest { limit });
        let response = client
            .get_latest_news(request)
            .await
            .map_err(|e| DomainError::InfrastructureError(format!("gRPC error: {}", e)))?;
        let news = response
            .into_inner()
            .news
            .into_iter()
            .map(|n| zukmove_core::domain::entities::news::News {
                id: n.id,
                name: n.name,
                source: n.source,
                date: n.date,
                tags: n.tags,
                city: n.city,
                country: n.country,
            })
            .collect();
        Ok(news)
    }

    async fn get_latest_news_in_city(
        &self,
        city: &str,
        limit: i32,
    ) -> Result<Vec<zukmove_core::domain::entities::news::News>, DomainError> {
        let mut client = self.client.clone();
        let request = tonic::Request::new(GetLatestNewsInCityRequest {
            city: city.to_string(),
            limit,
        });
        let response = client
            .get_latest_news_in_city(request)
            .await
            .map_err(|e| DomainError::InfrastructureError(format!("gRPC error: {}", e)))?;
        let news = response
            .into_inner()
            .news
            .into_iter()
            .map(|n| zukmove_core::domain::entities::news::News {
                id: n.id,
                name: n.name,
                source: n.source,
                date: n.date,
                tags: n.tags,
                city: n.city,
                country: n.country,
            })
            .collect();
        Ok(news)
    }

    async fn get_city_score(
        &self,
        city: &str,
    ) -> Result<Option<zukmove_core::domain::entities::city_score::CityScore>, DomainError> {
        let mut client = self.client.clone();
        let request = tonic::Request::new(crate::proto::GetCityScoreRequest {
            city: city.to_string(),
        });
        match client.get_city_score(request).await {
            Ok(response) => {
                let s = response.into_inner();
                // We return a mapped CityScore.
                // If it were not found we would return Ok(None) or Err,
                // but currently the Grpc MI8 server returns city score (or creates base 1000).
                Ok(Some(
                    zukmove_core::domain::entities::city_score::CityScore {
                        city: s.city,
                        country: s.country,
                        updated_at: s.updated_at,
                        quality_of_life: s.quality_of_life,
                        safety: s.safety,
                        economy: s.economy,
                        culture: s.culture,
                    },
                ))
            }
            Err(_) => Ok(None),
        }
    }
}
