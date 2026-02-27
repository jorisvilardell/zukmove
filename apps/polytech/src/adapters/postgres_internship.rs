use async_trait::async_trait;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use zukmove_core::domain::entities::internship::{Internship, InternshipStatus};
use zukmove_core::domain::ports::{DomainError, InternshipRepository};

#[derive(Debug, FromRow)]
struct InternshipRow {
    id: Uuid,
    student_id: Uuid,
    offer_id: Uuid,
    status: String,
    message: String,
}

impl TryFrom<InternshipRow> for Internship {
    type Error = DomainError;

    fn try_from(row: InternshipRow) -> Result<Self, Self::Error> {
        let status: InternshipStatus = row
            .status
            .parse()
            .map_err(|e: String| DomainError::InfrastructureError(e))?;
        Ok(Internship {
            id: row.id,
            student_id: row.student_id,
            offer_id: row.offer_id,
            status,
            message: row.message,
        })
    }
}

pub struct PostgresInternshipRepository {
    pool: PgPool,
}

impl PostgresInternshipRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InternshipRepository for PostgresInternshipRepository {
    async fn save(&self, internship: &Internship) -> Result<Internship, DomainError> {
        let status_str = internship.status.to_string();
        let row = sqlx::query_as::<_, InternshipRow>(
            "INSERT INTO internships (id, student_id, offer_id, status, message) VALUES ($1, $2, $3, $4, $5) RETURNING id, student_id, offer_id, status, message",
        )
        .bind(internship.id)
        .bind(internship.student_id)
        .bind(internship.offer_id)
        .bind(&status_str)
        .bind(&internship.message)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Internship::try_from(row)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Internship, DomainError> {
        let row = sqlx::query_as::<_, InternshipRow>(
            "SELECT id, student_id, offer_id, status, message FROM internships WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
        .ok_or_else(|| DomainError::NotFound(format!("Internship with id {} not found", id)))?;

        Internship::try_from(row)
    }
}
