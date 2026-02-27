use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub enum InternshipStatus {
    Approved,
    Rejected,
}

impl std::fmt::Display for InternshipStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternshipStatus::Approved => write!(f, "Approved"),
            InternshipStatus::Rejected => write!(f, "Rejected"),
        }
    }
}

impl std::str::FromStr for InternshipStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Approved" => Ok(InternshipStatus::Approved),
            "Rejected" => Ok(InternshipStatus::Rejected),
            _ => Err(format!("Invalid internship status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Internship {
    pub id: Uuid,
    pub student_id: Uuid,
    pub offer_id: Uuid,
    pub status: InternshipStatus,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateInternshipRequest {
    pub student_id: Uuid,
    pub offer_id: Uuid,
}
