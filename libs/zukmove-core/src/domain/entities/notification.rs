use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Notification {
    pub id: Uuid,
    pub student_id: Uuid,
    #[serde(rename = "type")]
    pub type_: String,
    pub offer_id: Uuid,
    pub message: String,
    pub read: bool,
}
