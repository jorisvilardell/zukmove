use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Offer {
    pub id: Uuid,
    pub title: String,
    pub link: String,
    pub city: String,
    pub domain: String,
    pub salary: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateOfferRequest {
    pub title: String,
    pub link: String,
    pub city: String,
    pub domain: String,
    pub salary: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateOfferRequest {
    pub title: Option<String>,
    pub link: Option<String>,
    pub city: Option<String>,
    pub domain: Option<String>,
    pub salary: Option<f64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub available: Option<bool>,
}
