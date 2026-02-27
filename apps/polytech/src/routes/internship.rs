use actix_web::{HttpResponse, web};
use uuid::Uuid;

use zukmove_core::domain::entities::internship::{CreateInternshipRequest, Internship};

use crate::AppState;
use crate::routes::student::domain_error_to_response;

#[utoipa::path(
    post,
    path = "/internship",
    request_body = CreateInternshipRequest,
    responses(
        (status = 201, description = "Internship status created", body = Internship),
        (status = 404, description = "Student or Offer not found"),
        (status = 500, description = "Internal error")
    )
)]
pub async fn create_internship(
    state: web::Data<AppState>,
    body: web::Json<CreateInternshipRequest>,
) -> HttpResponse {
    match state.internship_service.register(body.into_inner()).await {
        Ok(internship) => HttpResponse::Created().json(internship),
        Err(e) => domain_error_to_response(e),
    }
}

#[utoipa::path(
    get,
    path = "/internship/{id}",
    params(
        ("id" = Uuid, Path, description = "Internship ID")
    ),
    responses(
        (status = 200, description = "Internship found", body = Internship),
        (status = 404, description = "Internship not found"),
        (status = 500, description = "Internal error")
    )
)]
pub async fn get_internship(state: web::Data<AppState>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    match state.internship_service.find_by_id(id).await {
        Ok(internship) => HttpResponse::Ok().json(internship),
        Err(e) => domain_error_to_response(e),
    }
}
