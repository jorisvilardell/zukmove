use async_trait::async_trait;
use uuid::Uuid;

use super::entities::internship::Internship;
use super::entities::offer::Offer;
use super::entities::student::Student;

/// Domain error type
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),
}

// ─── Student Repository (Port) ───

#[async_trait]
pub trait StudentRepository: Send + Sync {
    async fn save(&self, student: &Student) -> Result<Student, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Student, DomainError>;
    async fn find_by_domain(&self, domain: &str) -> Result<Vec<Student>, DomainError>;
    async fn update(&self, student: &Student) -> Result<Student, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

// ─── Offer Repository (Port) ───

#[async_trait]
pub trait OfferRepository: Send + Sync {
    async fn save(&self, offer: &Offer) -> Result<Offer, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Offer, DomainError>;
    async fn find_by_domain(&self, domain: &str) -> Result<Vec<Offer>, DomainError>;
    async fn find_by_city(&self, city: &str) -> Result<Vec<Offer>, DomainError>;
    async fn update(&self, offer: &Offer) -> Result<Offer, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

// ─── Internship Repository (Port) ───

#[async_trait]
pub trait InternshipRepository: Send + Sync {
    async fn save(&self, internship: &Internship) -> Result<Internship, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Internship, DomainError>;
}

// ─── Offer Client (Port sortant pour communication inter-services) ───

#[async_trait]
pub trait OfferClient: Send + Sync {
    async fn get_offer_by_id(&self, id: Uuid) -> Result<Offer, DomainError>;
}
