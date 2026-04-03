use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct News {
    pub id: String,
    pub name: String,
    pub source: String,
    pub date: String,
    pub tags: Vec<String>,
    pub city: String,
    pub country: String,
}
