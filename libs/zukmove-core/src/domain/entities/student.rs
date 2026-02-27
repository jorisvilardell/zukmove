use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Student {
    pub id: Uuid,
    pub firstname: String,
    pub name: String,
    pub domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateStudentRequest {
    pub firstname: String,
    pub name: String,
    pub domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateStudentRequest {
    pub firstname: Option<String>,
    pub name: Option<String>,
    pub domain: Option<String>,
}
