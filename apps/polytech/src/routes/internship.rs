use actix_web::{web, HttpResponse};
use uuid::Uuid;

use zukmove_core::domain::entities::internship::CreateInternshipRequest;

use crate::routes::student::domain_error_to_response;
use crate::AppState;

pub async fn create_internship(
    state: web::Data<AppState>,
    body: web::Json<CreateInternshipRequest>,
) -> HttpResponse {
    match state.internship_service.register(body.into_inner()).await {
        Ok(internship) => HttpResponse::Created().json(internship),
        Err(e) => domain_error_to_response(e),
    }
}

pub async fn get_internship(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let id = path.into_inner();
    match state.internship_service.find_by_id(id).await {
        Ok(internship) => HttpResponse::Ok().json(internship),
        Err(e) => domain_error_to_response(e),
    }
}
