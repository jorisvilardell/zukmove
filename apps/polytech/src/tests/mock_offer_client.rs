use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use zukmove_core::domain::entities::offer::Offer;
use zukmove_core::domain::ports::{DomainError, OfferClient};

/// Mock OfferClient that simulates responses from Erasmumu.
pub struct MockOfferClient {
    offers: Mutex<HashMap<Uuid, Offer>>,
}

impl MockOfferClient {
    pub fn new() -> Self {
        Self {
            offers: Mutex::new(HashMap::new()),
        }
    }

    /// Pre-load an offer for the mock to return.
    pub fn add_offer(&self, offer: Offer) {
        let mut store = self.offers.lock().unwrap();
        store.insert(offer.id, offer);
    }
}

#[async_trait]
impl OfferClient for MockOfferClient {
    async fn get_offer_by_id(&self, id: Uuid) -> Result<Offer, DomainError> {
        let store = self.offers.lock().unwrap();
        store
            .get(&id)
            .cloned()
            .ok_or_else(|| DomainError::NotFound(format!("Offer with id {} not found", id)))
    }
}

/// Helper to create a test offer.
pub fn make_test_offer(id: Uuid, domain: &str) -> Offer {
    Offer {
        id,
        title: "Stage Test".to_string(),
        link: "http://example.com".to_string(),
        city: "Paris".to_string(),
        domain: domain.to_string(),
        salary: 1200.0,
        start_date: NaiveDate::from_ymd_opt(2026, 6, 1).unwrap(),
        end_date: NaiveDate::from_ymd_opt(2026, 12, 1).unwrap(),
        available: true,
    }
}
