use async_trait::async_trait;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use zukmove_core::domain::entities::student::Student;
use zukmove_core::domain::ports::{DomainError, StudentRepository};

#[derive(Debug, FromRow)]
struct StudentRow {
    id: Uuid,
    firstname: String,
    name: String,
    domain: String,
}

impl From<StudentRow> for Student {
    fn from(row: StudentRow) -> Self {
        Student {
            id: row.id,
            firstname: row.firstname,
            name: row.name,
            domain: row.domain,
        }
    }
}

pub struct PostgresStudentRepository {
    pool: PgPool,
}

impl PostgresStudentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StudentRepository for PostgresStudentRepository {
    async fn save(&self, student: &Student) -> Result<Student, DomainError> {
        let row = sqlx::query_as::<_, StudentRow>(
            "INSERT INTO students (id, firstname, name, domain) VALUES ($1, $2, $3, $4) RETURNING id, firstname, name, domain",
        )
        .bind(student.id)
        .bind(&student.firstname)
        .bind(&student.name)
        .bind(&student.domain)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(row.into())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Student, DomainError> {
        let row = sqlx::query_as::<_, StudentRow>(
            "SELECT id, firstname, name, domain FROM students WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
        .ok_or_else(|| DomainError::NotFound(format!("Student with id {} not found", id)))?;

        Ok(row.into())
    }

    async fn find_by_domain(&self, domain: &str) -> Result<Vec<Student>, DomainError> {
        let rows = sqlx::query_as::<_, StudentRow>(
            "SELECT id, firstname, name, domain FROM students WHERE domain = $1",
        )
        .bind(domain)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn update(&self, student: &Student) -> Result<Student, DomainError> {
        let row = sqlx::query_as::<_, StudentRow>(
            "UPDATE students SET firstname = $2, name = $3, domain = $4 WHERE id = $1 RETURNING id, firstname, name, domain",
        )
        .bind(student.id)
        .bind(&student.firstname)
        .bind(&student.name)
        .bind(&student.domain)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
        .ok_or_else(|| DomainError::NotFound(format!("Student with id {} not found", student.id)))?;

        Ok(row.into())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query("DELETE FROM students WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound(format!(
                "Student with id {} not found",
                id
            )));
        }
        Ok(())
    }
}
