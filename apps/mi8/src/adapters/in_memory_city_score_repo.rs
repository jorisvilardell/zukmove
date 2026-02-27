use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use zukmove_core::domain::entities::city_score::CityScore;
use zukmove_core::domain::ports::{CityScoreRepository, DomainError};

/// In-memory CityScoreRepository for Phase 1 validation.
pub struct InMemoryCityScoreRepository {
    store: Mutex<HashMap<String, CityScore>>,
}

impl InMemoryCityScoreRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl CityScoreRepository for InMemoryCityScoreRepository {
    async fn get_or_create(&self, city: &str, country: &str) -> Result<CityScore, DomainError> {
        let mut store = self.store.lock().unwrap();
        let key = city.to_lowercase();
        if let Some(score) = store.get(&key) {
            Ok(score.clone())
        } else {
            let score = CityScore::new(city, country);
            store.insert(key, score.clone());
            Ok(score)
        }
    }

    async fn save(&self, score: &CityScore) -> Result<CityScore, DomainError> {
        let mut store = self.store.lock().unwrap();
        let key = score.city.to_lowercase();
        store.insert(key, score.clone());
        Ok(score.clone())
    }

    async fn get_top_cities(&self, limit: usize) -> Result<Vec<CityScore>, DomainError> {
        let store = self.store.lock().unwrap();
        let mut scores: Vec<CityScore> = store.values().cloned().collect();
        scores.sort_by(|a, b| a.total_score().cmp(&b.total_score())); // ascending
        Ok(scores.into_iter().take(limit).collect())
    }
}
