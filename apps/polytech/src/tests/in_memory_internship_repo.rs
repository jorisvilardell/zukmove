use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use zukmove_core::domain::entities::internship::Internship;
use zukmove_core::domain::ports::{DomainError, InternshipRepository};

/// In-memory implementation of InternshipRepository for testing.
pub struct InMemoryInternshipRepository {
    store: Mutex<HashMap<Uuid, Internship>>,
}

impl InMemoryInternshipRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl InternshipRepository for InMemoryInternshipRepository {
    async fn save(&self, internship: &Internship) -> Result<Internship, DomainError> {
        let mut store = self.store.lock().unwrap();
        store.insert(internship.id, internship.clone());
        Ok(internship.clone())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Internship, DomainError> {
        let store = self.store.lock().unwrap();
        store
            .get(&id)
            .cloned()
            .ok_or_else(|| DomainError::NotFound(format!("Internship with id {} not found", id)))
    }
}
