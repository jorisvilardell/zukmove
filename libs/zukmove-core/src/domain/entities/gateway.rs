use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::city_score::CityScore;
use super::news::News;
use super::offer::Offer;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct AggregatedOffer {
    pub offer: Offer,
    pub city_score: Option<CityScore>,
    pub latest_news: Option<Vec<News>>,
}
