use actix_web::{HttpResponse, web};
use uuid::Uuid;

use zukmove_core::domain::entities::offer::{CreateOfferRequest, Offer, UpdateOfferRequest};
use zukmove_core::domain::ports::DomainError;

use crate::AppState;

pub async fn create_offer(
    state: web::Data<AppState>,
    body: web::Json<CreateOfferRequest>,
) -> HttpResponse {
    let offer = Offer {
        id: Uuid::new_v4(),
        title: body.title.clone(),
        link: body.link.clone(),
        city: body.city.clone(),
        domain: body.domain.clone(),
        salary: body.salary,
        start_date: body.start_date,
        end_date: body.end_date,
        available: body.available,
    };

    match state.offer_repo.save(&offer).await {
        Ok(o) => HttpResponse::Created().json(o),
        Err(e) => domain_error_to_response(e),
    }
}

pub async fn get_offer(state: web::Data<AppState>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    match state.offer_repo.find_by_id(id).await {
        Ok(o) => HttpResponse::Ok().json(o),
        Err(e) => domain_error_to_response(e),
    }
}

#[derive(serde::Deserialize)]
pub struct OfferQuery {
    pub domain: Option<String>,
    pub city: Option<String>,
}

pub async fn list_offers(
    state: web::Data<AppState>,
    query: web::Query<OfferQuery>,
) -> HttpResponse {
    if let Some(ref domain) = query.domain {
        match state.offer_repo.find_by_domain(domain).await {
            Ok(offers) => HttpResponse::Ok().json(offers),
            Err(e) => domain_error_to_response(e),
        }
    } else if let Some(ref city) = query.city {
        match state.offer_repo.find_by_city(city).await {
            Ok(offers) => HttpResponse::Ok().json(offers),
            Err(e) => domain_error_to_response(e),
        }
    } else {
        HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Query parameter 'domain' or 'city' is required"
        }))
    }
}

pub async fn update_offer(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateOfferRequest>,
) -> HttpResponse {
    let id = path.into_inner();

    // Retrieve existing — note: find_by_id enforces available=true,
    // but for update we need to find even unavailable offers.
    // We'll try to find it and if not found treat as 404.
    let existing = match state.offer_repo.find_by_id(id).await {
        Ok(o) => o,
        Err(DomainError::NotFound(_)) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": format!("Offer with id {} not found", id)
            }));
        }
        Err(e) => return domain_error_to_response(e),
    };

    let updated = Offer {
        id: existing.id,
        title: body.title.clone().unwrap_or(existing.title),
        link: body.link.clone().unwrap_or(existing.link),
        city: body.city.clone().unwrap_or(existing.city),
        domain: body.domain.clone().unwrap_or(existing.domain),
        salary: body.salary.unwrap_or(existing.salary),
        start_date: body.start_date.unwrap_or(existing.start_date),
        end_date: body.end_date.unwrap_or(existing.end_date),
        available: body.available.unwrap_or(existing.available),
    };

    match state.offer_repo.update(&updated).await {
        Ok(o) => HttpResponse::Ok().json(o),
        Err(e) => domain_error_to_response(e),
    }
}

pub async fn delete_offer(state: web::Data<AppState>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    match state.offer_repo.delete(id).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => domain_error_to_response(e),
    }
}

fn domain_error_to_response(err: DomainError) -> HttpResponse {
    match err {
        DomainError::NotFound(msg) => HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        })),
        DomainError::ValidationError(msg) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": msg
        })),
        DomainError::InfrastructureError(msg) => {
            log::error!("Infrastructure error: {}", msg);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            }))
        }
    }
}
