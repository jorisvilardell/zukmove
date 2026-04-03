use actix_web::{HttpResponse, web};

use crate::AppState;
use zukmove_core::domain::entities::gateway::AggregatedOffer;

#[derive(serde::Deserialize)]
pub struct OfferQuery {
    pub domain: Option<String>,
    pub city: Option<String>,
}

#[utoipa::path(
    get,
    path = "/offer",
    params(
        ("domain" = Option<String>, Query, description = "Filter by domain (optional)"),
        ("city" = Option<String>, Query, description = "Filter by city (optional)")
    ),
    responses(
        (status = 200, description = "List of aggregated offers", body = Vec<AggregatedOffer>),
        (status = 500, description = "Internal error")
    )
)]
pub async fn get_offers(state: web::Data<AppState>, query: web::Query<OfferQuery>) -> HttpResponse {
    let domain = query.domain.clone();
    let city = query.city.clone();

    match state
        .internship_service
        .get_aggregated_offers(domain, city)
        .await
    {
        Ok(offers) => HttpResponse::Ok().json(offers),
        Err(e) => crate::routes::student::domain_error_to_response(e),
    }
}
