use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use zukmove_core::domain::entities::notification::Notification;
use zukmove_core::domain::ports::{DomainError, NotificationRepository};

pub struct PostgresNotificationRepository {
    pool: PgPool,
}

impl PostgresNotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct NotificationRow {
    id: Uuid,
    student_id: Uuid,
    #[sqlx(rename = "type")]
    type_: String,
    offer_id: Uuid,
    message: String,
    read: bool,
}

impl From<NotificationRow> for Notification {
    fn from(row: NotificationRow) -> Self {
        Notification {
            id: row.id,
            student_id: row.student_id,
            type_: row.type_,
            offer_id: row.offer_id,
            message: row.message,
            read: row.read,
        }
    }
}

#[async_trait]
impl NotificationRepository for PostgresNotificationRepository {
    async fn save(&self, notification: &Notification) -> Result<Notification, DomainError> {
        sqlx::query(
            "INSERT INTO notifications (id, student_id, type, offer_id, message, read) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(notification.id)
        .bind(notification.student_id)
        .bind(&notification.type_)
        .bind(notification.offer_id)
        .bind(&notification.message)
        .bind(notification.read)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(notification.clone())
    }

    async fn find_by_student_id(&self, student_id: Uuid) -> Result<Vec<Notification>, DomainError> {
        let rows = sqlx::query_as::<_, NotificationRow>(
            "SELECT id, student_id, type, offer_id, message, read FROM notifications WHERE student_id = $1 ORDER BY id DESC",
        )
        .bind(student_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn mark_as_read(&self, id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query("UPDATE notifications SET read = true WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound(format!(
                "Notification {} not found",
                id
            )));
        }

        Ok(())
    }
}
