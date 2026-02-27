use std::sync::Mutex;

use async_trait::async_trait;
use zukmove_core::domain::entities::news::News;
use zukmove_core::domain::ports::{DomainError, NewsRepository};

/// In-memory NewsRepository backed by a Vec, with hardcoded fake data.
pub struct ArrayNewsRepository {
    store: Mutex<Vec<News>>,
}

impl ArrayNewsRepository {
    pub fn new() -> Self {
        let fake_data = vec![
            News {
                id: "1".to_string(),
                name: "Tech Innovation Hub Opens in Paris".to_string(),
                source: "TechNews".to_string(),
                date: "2026-02-20".to_string(),
                tags: vec!["innovation".to_string(), "economy".to_string()],
                city: "Paris".to_string(),
                country: "France".to_string(),
            },
            News {
                id: "2".to_string(),
                name: "Berlin Festival of Lights".to_string(),
                source: "CultureDaily".to_string(),
                date: "2026-02-19".to_string(),
                tags: vec!["festival".to_string(), "tourism".to_string(), "culture".to_string()],
                city: "Berlin".to_string(),
                country: "Germany".to_string(),
            },
            News {
                id: "3".to_string(),
                name: "New University Campus in Barcelona".to_string(),
                source: "EduWorld".to_string(),
                date: "2026-02-18".to_string(),
                tags: vec!["education".to_string(), "innovation".to_string()],
                city: "Barcelona".to_string(),
                country: "Spain".to_string(),
            },
            News {
                id: "4".to_string(),
                name: "Paris Air Quality Concerns Rise".to_string(),
                source: "EnvReport".to_string(),
                date: "2026-02-17".to_string(),
                tags: vec!["pollution".to_string(), "health".to_string()],
                city: "Paris".to_string(),
                country: "France".to_string(),
            },
            News {
                id: "5".to_string(),
                name: "Berlin Sports Championship".to_string(),
                source: "SportsMag".to_string(),
                date: "2026-02-16".to_string(),
                tags: vec!["sports".to_string(), "tourism".to_string()],
                city: "Berlin".to_string(),
                country: "Germany".to_string(),
            },
        ];

        Self {
            store: Mutex::new(fake_data),
        }
    }
}

#[async_trait]
impl NewsRepository for ArrayNewsRepository {
    async fn save(&self, news: &News) -> Result<News, DomainError> {
        let mut store = self.store.lock().unwrap();
        store.insert(0, news.clone()); // Insert at front (newest first)
        Ok(news.clone())
    }

    async fn get_latest(&self, limit: usize) -> Result<Vec<News>, DomainError> {
        let store = self.store.lock().unwrap();
        Ok(store.iter().take(limit).cloned().collect())
    }

    async fn get_latest_in_city(
        &self,
        city: &str,
        limit: usize,
    ) -> Result<Vec<News>, DomainError> {
        let store = self.store.lock().unwrap();
        Ok(store
            .iter()
            .filter(|n| n.city.to_lowercase() == city.to_lowercase())
            .take(limit)
            .cloned()
            .collect())
    }
}
