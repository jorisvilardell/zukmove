mod in_memory_internship_repo;
mod in_memory_student_repo;
mod mock_offer_client;

use actix_web::{App, test, web};
use serde_json::json;
use uuid::Uuid;

use crate::AppState;
use crate::routes;
use in_memory_internship_repo::InMemoryInternshipRepository;
use in_memory_student_repo::InMemoryStudentRepository;
use mock_offer_client::{MockOfferClient, make_test_offer};
use zukmove_core::app::internship_service::InternshipService;
use zukmove_core::domain::entities::internship::Internship;
use zukmove_core::domain::entities::student::Student;
use zukmove_core::domain::ports::StudentRepository;

fn test_app_state() -> web::Data<AppState> {
    web::Data::new(AppState {
        student_repo: Box::new(InMemoryStudentRepository::new()),
        internship_service: InternshipService::new(
            Box::new(InMemoryStudentRepository::new()),
            Box::new(InMemoryInternshipRepository::new()),
            Box::new(MockOfferClient::new()),
        ),
    })
}

fn test_app(
    state: web::Data<AppState>,
) -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(state)
        .route("/student", web::post().to(routes::student::create_student))
        .route("/student", web::get().to(routes::student::list_students))
        .route("/student/{id}", web::get().to(routes::student::get_student))
        .route(
            "/student/{id}",
            web::put().to(routes::student::update_student),
        )
        .route(
            "/student/{id}",
            web::delete().to(routes::student::delete_student),
        )
        .route(
            "/internship",
            web::post().to(routes::internship::create_internship),
        )
        .route(
            "/internship/{id}",
            web::get().to(routes::internship::get_internship),
        )
}

// ─── POST /student ───

#[actix_web::test]
async fn test_create_student() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let payload = json!({
        "firstname": "Alice",
        "name": "Dupont",
        "domain": "IT"
    });

    let req = test::TestRequest::post()
        .uri("/student")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 201);

    let body: Student = test::read_body_json(resp).await;
    assert_eq!(body.firstname, "Alice");
    assert_eq!(body.name, "Dupont");
    assert_eq!(body.domain, "IT");
}

// ─── GET /student/{id} ───

#[actix_web::test]
async fn test_get_student_by_id() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    // Create
    let payload = json!({ "firstname": "Bob", "name": "Martin", "domain": "IT" });
    let req = test::TestRequest::post()
        .uri("/student")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    let created: Student = test::read_body_json(resp).await;

    // Get
    let req = test::TestRequest::get()
        .uri(&format!("/student/{}", created.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let body: Student = test::read_body_json(resp).await;
    assert_eq!(body.id, created.id);
    assert_eq!(body.firstname, "Bob");
}

#[actix_web::test]
async fn test_get_student_not_found() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let req = test::TestRequest::get()
        .uri(&format!("/student/{}", Uuid::new_v4()))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

// ─── GET /student?domain= ───

#[actix_web::test]
async fn test_list_students_by_domain() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    for (name, domain) in [("Alice", "IT"), ("Bob", "IT"), ("Claire", "life science")] {
        let payload = json!({ "firstname": name, "name": "Test", "domain": domain });
        let req = test::TestRequest::post()
            .uri("/student")
            .set_json(&payload)
            .to_request();
        test::call_service(&app, req).await;
    }

    let req = test::TestRequest::get()
        .uri("/student?domain=IT")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let body: Vec<Student> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 2);
}

#[actix_web::test]
async fn test_list_students_without_domain_returns_400() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let req = test::TestRequest::get().uri("/student").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

// ─── PUT /student/{id} ───

#[actix_web::test]
async fn test_update_student() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let payload = json!({ "firstname": "Alice", "name": "Dupont", "domain": "IT" });
    let req = test::TestRequest::post()
        .uri("/student")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    let created: Student = test::read_body_json(resp).await;

    // Update firstname only
    let update_payload = json!({ "firstname": "Alicia" });
    let req = test::TestRequest::put()
        .uri(&format!("/student/{}", created.id))
        .set_json(&update_payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let body: Student = test::read_body_json(resp).await;
    assert_eq!(body.firstname, "Alicia");
    assert_eq!(body.name, "Dupont"); // unchanged
}

// ─── DELETE /student/{id} ───

#[actix_web::test]
async fn test_delete_student() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let payload = json!({ "firstname": "Alice", "name": "Dupont", "domain": "IT" });
    let req = test::TestRequest::post()
        .uri("/student")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    let created: Student = test::read_body_json(resp).await;

    // Delete
    let req = test::TestRequest::delete()
        .uri(&format!("/student/{}", created.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 204);

    // Verify gone
    let req = test::TestRequest::get()
        .uri(&format!("/student/{}", created.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_delete_student_not_found() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let req = test::TestRequest::delete()
        .uri(&format!("/student/{}", Uuid::new_v4()))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

// ─── POST /internship (domain match → approved) ───

#[actix_web::test]
async fn test_internship_approved_when_domains_match() {
    // Build state with shared repos so student + mock offer share the same data
    let student_repo = InMemoryStudentRepository::new();
    let student_repo_for_service = InMemoryStudentRepository::new();
    let internship_repo = InMemoryInternshipRepository::new();
    let offer_client = MockOfferClient::new();

    // Pre-load a student in both repos (routes repo + service repo)
    let student = Student {
        id: Uuid::new_v4(),
        firstname: "Alice".to_string(),
        name: "Dupont".to_string(),
        domain: "IT".to_string(),
    };
    student_repo.save(&student).await.unwrap();
    student_repo_for_service.save(&student).await.unwrap();

    // Pre-load an offer in the mock client with matching domain
    let offer_id = Uuid::new_v4();
    offer_client.add_offer(make_test_offer(offer_id, "IT"));

    let state = web::Data::new(AppState {
        student_repo: Box::new(student_repo),
        internship_service: InternshipService::new(
            Box::new(student_repo_for_service),
            Box::new(internship_repo),
            Box::new(offer_client),
        ),
    });

    let app = test::init_service(test_app(state)).await;

    let payload = json!({
        "student_id": student.id,
        "offer_id": offer_id
    });

    let req = test::TestRequest::post()
        .uri("/internship")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 201);

    let body: Internship = test::read_body_json(resp).await;
    assert_eq!(
        body.status,
        zukmove_core::domain::entities::internship::InternshipStatus::Approved
    );
}

// ─── POST /internship (domain mismatch → rejected) ───

#[actix_web::test]
async fn test_internship_rejected_when_domains_dont_match() {
    let student_repo = InMemoryStudentRepository::new();
    let student_repo_for_service = InMemoryStudentRepository::new();
    let internship_repo = InMemoryInternshipRepository::new();
    let offer_client = MockOfferClient::new();

    let student = Student {
        id: Uuid::new_v4(),
        firstname: "Bob".to_string(),
        name: "Martin".to_string(),
        domain: "IT".to_string(),
    };
    student_repo.save(&student).await.unwrap();
    student_repo_for_service.save(&student).await.unwrap();

    // Offer with different domain
    let offer_id = Uuid::new_v4();
    offer_client.add_offer(make_test_offer(offer_id, "life science"));

    let state = web::Data::new(AppState {
        student_repo: Box::new(student_repo),
        internship_service: InternshipService::new(
            Box::new(student_repo_for_service),
            Box::new(internship_repo),
            Box::new(offer_client),
        ),
    });

    let app = test::init_service(test_app(state)).await;

    let payload = json!({
        "student_id": student.id,
        "offer_id": offer_id
    });

    let req = test::TestRequest::post()
        .uri("/internship")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 201);

    let body: Internship = test::read_body_json(resp).await;
    assert_eq!(
        body.status,
        zukmove_core::domain::entities::internship::InternshipStatus::Rejected
    );
    assert!(body.message.contains("ne correspond pas"));
}

// ─── GET /internship/{id} ───

#[actix_web::test]
async fn test_get_internship_not_found() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let req = test::TestRequest::get()
        .uri(&format!("/internship/{}", Uuid::new_v4()))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

// ─── POST /internship with non-existent student ───

#[actix_web::test]
async fn test_internship_with_unknown_student_returns_404() {
    let offer_client = MockOfferClient::new();
    let offer_id = Uuid::new_v4();
    offer_client.add_offer(make_test_offer(offer_id, "IT"));

    let state = web::Data::new(AppState {
        student_repo: Box::new(InMemoryStudentRepository::new()),
        internship_service: InternshipService::new(
            Box::new(InMemoryStudentRepository::new()),
            Box::new(InMemoryInternshipRepository::new()),
            Box::new(offer_client),
        ),
    });

    let app = test::init_service(test_app(state)).await;

    let payload = json!({
        "student_id": Uuid::new_v4(),
        "offer_id": offer_id
    });

    let req = test::TestRequest::post()
        .uri("/internship")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

// ─── POST /internship with non-existent offer ───

#[actix_web::test]
async fn test_internship_with_unknown_offer_returns_404() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let payload = json!({
        "student_id": Uuid::new_v4(),
        "offer_id": Uuid::new_v4()
    });

    let req = test::TestRequest::post()
        .uri("/internship")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}
