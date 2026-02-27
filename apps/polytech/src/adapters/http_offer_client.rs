use async_trait::async_trait;
use uuid::Uuid;

use zukmove_core::domain::entities::offer::Offer;
use zukmove_core::domain::ports::{DomainError, OfferClient};

pub struct HttpOfferClient {
    base_url: String,
    client: reqwest::Client,
}

impl HttpOfferClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl OfferClient for HttpOfferClient {
    async fn get_offer_by_id(&self, id: Uuid) -> Result<Offer, DomainError> {
        let url = format!("{}/offer/{}", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| DomainError::InfrastructureError(format!("HTTP request failed: {}", e)))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(DomainError::NotFound(format!(
                "Offer with id {} not found on Erasmumu",
                id
            )));
        }

        if !response.status().is_success() {
            return Err(DomainError::InfrastructureError(format!(
                "Erasmumu returned status {}",
                response.status()
            )));
        }

        response
            .json::<Offer>()
            .await
            .map_err(|e| DomainError::InfrastructureError(format!("Failed to parse offer: {}", e)))
    }
}
