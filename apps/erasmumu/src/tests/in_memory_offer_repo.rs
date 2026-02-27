use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use zukmove_core::domain::entities::offer::Offer;
use zukmove_core::domain::ports::{DomainError, OfferRepository};

/// In-memory implementation of OfferRepository for testing.
pub struct InMemoryOfferRepository {
    store: Mutex<HashMap<Uuid, Offer>>,
}

impl InMemoryOfferRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl OfferRepository for InMemoryOfferRepository {
    async fn save(&self, offer: &Offer) -> Result<Offer, DomainError> {
        let mut store = self.store.lock().unwrap();
        store.insert(offer.id, offer.clone());
        Ok(offer.clone())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Offer, DomainError> {
        let store = self.store.lock().unwrap();
        let offer = store
            .get(&id)
            .ok_or_else(|| DomainError::NotFound(format!("Offer with id {} not found", id)))?
            .clone();

        // Business rule: filter unavailable offers
        if !offer.available {
            return Err(DomainError::NotFound(format!(
                "Offer with id {} is not available",
                id
            )));
        }
        Ok(offer)
    }

    async fn find_by_domain(&self, domain: &str) -> Result<Vec<Offer>, DomainError> {
        let store = self.store.lock().unwrap();
        Ok(store
            .values()
            .filter(|o| o.domain == domain && o.available)
            .cloned()
            .collect())
    }

    async fn find_by_city(&self, city: &str) -> Result<Vec<Offer>, DomainError> {
        let store = self.store.lock().unwrap();
        Ok(store
            .values()
            .filter(|o| o.city == city && o.available)
            .cloned()
            .collect())
    }

    async fn update(&self, offer: &Offer) -> Result<Offer, DomainError> {
        let mut store = self.store.lock().unwrap();
        if !store.contains_key(&offer.id) {
            return Err(DomainError::NotFound(format!(
                "Offer with id {} not found",
                offer.id
            )));
        }
        store.insert(offer.id, offer.clone());
        Ok(offer.clone())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        let mut store = self.store.lock().unwrap();
        store
            .remove(&id)
            .ok_or_else(|| DomainError::NotFound(format!("Offer with id {} not found", id)))?;
        Ok(())
    }
}
