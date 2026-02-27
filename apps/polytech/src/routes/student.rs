use actix_web::{HttpResponse, web};
use uuid::Uuid;

use zukmove_core::domain::entities::student::{
    CreateStudentRequest, Student, UpdateStudentRequest,
};
use zukmove_core::domain::ports::DomainError;

use crate::AppState;

pub async fn create_student(
    state: web::Data<AppState>,
    body: web::Json<CreateStudentRequest>,
) -> HttpResponse {
    let student = Student {
        id: Uuid::new_v4(),
        firstname: body.firstname.clone(),
        name: body.name.clone(),
        domain: body.domain.clone(),
    };

    match state.student_repo.save(&student).await {
        Ok(s) => HttpResponse::Created().json(s),
        Err(e) => domain_error_to_response(e),
    }
}

pub async fn get_student(state: web::Data<AppState>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    match state.student_repo.find_by_id(id).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err(e) => domain_error_to_response(e),
    }
}

#[derive(serde::Deserialize)]
pub struct StudentQuery {
    pub domain: Option<String>,
}

pub async fn list_students(
    state: web::Data<AppState>,
    query: web::Query<StudentQuery>,
) -> HttpResponse {
    if let Some(ref domain) = query.domain {
        match state.student_repo.find_by_domain(domain).await {
            Ok(students) => HttpResponse::Ok().json(students),
            Err(e) => domain_error_to_response(e),
        }
    } else {
        HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Query parameter 'domain' is required"
        }))
    }
}

pub async fn update_student(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateStudentRequest>,
) -> HttpResponse {
    let id = path.into_inner();

    // Retrieve existing student first
    let existing = match state.student_repo.find_by_id(id).await {
        Ok(s) => s,
        Err(e) => return domain_error_to_response(e),
    };

    let updated = Student {
        id: existing.id,
        firstname: body.firstname.clone().unwrap_or(existing.firstname),
        name: body.name.clone().unwrap_or(existing.name),
        domain: body.domain.clone().unwrap_or(existing.domain),
    };

    match state.student_repo.update(&updated).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err(e) => domain_error_to_response(e),
    }
}

pub async fn delete_student(state: web::Data<AppState>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    match state.student_repo.delete(id).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => domain_error_to_response(e),
    }
}

pub fn domain_error_to_response(err: DomainError) -> HttpResponse {
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
