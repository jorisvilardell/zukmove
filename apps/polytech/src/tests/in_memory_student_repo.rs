use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use zukmove_core::domain::entities::student::Student;
use zukmove_core::domain::ports::{DomainError, StudentRepository};

/// In-memory implementation of StudentRepository for testing.
pub struct InMemoryStudentRepository {
    store: Mutex<HashMap<Uuid, Student>>,
}

impl InMemoryStudentRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl StudentRepository for InMemoryStudentRepository {
    async fn save(&self, student: &Student) -> Result<Student, DomainError> {
        let mut store = self.store.lock().unwrap();
        store.insert(student.id, student.clone());
        Ok(student.clone())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Student, DomainError> {
        let store = self.store.lock().unwrap();
        store
            .get(&id)
            .cloned()
            .ok_or_else(|| DomainError::NotFound(format!("Student with id {} not found", id)))
    }

    async fn find_by_domain(&self, domain: &str) -> Result<Vec<Student>, DomainError> {
        let store = self.store.lock().unwrap();
        Ok(store
            .values()
            .filter(|s| s.domain == domain)
            .cloned()
            .collect())
    }

    async fn update(&self, student: &Student) -> Result<Student, DomainError> {
        let mut store = self.store.lock().unwrap();
        if !store.contains_key(&student.id) {
            return Err(DomainError::NotFound(format!(
                "Student with id {} not found",
                student.id
            )));
        }
        store.insert(student.id, student.clone());
        Ok(student.clone())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        let mut store = self.store.lock().unwrap();
        store
            .remove(&id)
            .ok_or_else(|| DomainError::NotFound(format!("Student with id {} not found", id)))?;
        Ok(())
    }
}
