use actix_web::{HttpResponse, web};
use uuid::Uuid;

use zukmove_core::domain::ports::NotificationRepository;

use crate::AppState;
use crate::routes::student::domain_error_to_response;

pub async fn get_notifications(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let student_id = path.into_inner();
    match state.notification_repo.find_by_student_id(student_id).await {
        Ok(notifications) => HttpResponse::Ok().json(notifications),
        Err(e) => domain_error_to_response(e),
    }
}

pub async fn mark_as_read(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let id = path.into_inner();
    match state.notification_repo.mark_as_read(id).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({"status": "ok"})),
        Err(e) => domain_error_to_response(e),
    }
}
