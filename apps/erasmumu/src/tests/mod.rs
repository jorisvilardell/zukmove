mod in_memory_offer_repo;

use actix_web::{App, test, web};
use serde_json::json;

use crate::AppState;
use crate::routes;
use in_memory_offer_repo::InMemoryOfferRepository;
use zukmove_core::domain::entities::offer::Offer;

fn test_app_state() -> web::Data<AppState> {
    web::Data::new(AppState {
        offer_repo: Box::new(InMemoryOfferRepository::new()),
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
        .route("/offer", web::post().to(routes::offer::create_offer))
        .route("/offer", web::get().to(routes::offer::list_offers))
        .route("/offer/{id}", web::get().to(routes::offer::get_offer))
        .route("/offer/{id}", web::put().to(routes::offer::update_offer))
        .route("/offer/{id}", web::delete().to(routes::offer::delete_offer))
}

// ─── POST /offer ───

#[actix_web::test]
async fn test_create_offer() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let payload = json!({
        "title": "Stage Dev",
        "link": "http://example.com",
        "city": "Paris",
        "domain": "IT",
        "salary": 1200.0,
        "start_date": "2026-06-01",
        "end_date": "2026-12-01",
        "available": true
    });

    let req = test::TestRequest::post()
        .uri("/offer")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 201);

    let body: Offer = test::read_body_json(resp).await;
    assert_eq!(body.title, "Stage Dev");
    assert_eq!(body.city, "Paris");
    assert_eq!(body.domain, "IT");
}

// ─── GET /offer/{id} ───

#[actix_web::test]
async fn test_get_offer_by_id() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    // Create an offer first
    let payload = json!({
        "title": "Stage Data",
        "link": "http://example.com",
        "city": "Lyon",
        "domain": "IT",
        "salary": 1500.0,
        "start_date": "2026-06-01",
        "end_date": "2026-12-01",
        "available": true
    });

    let req = test::TestRequest::post()
        .uri("/offer")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    let created: Offer = test::read_body_json(resp).await;

    // Now get it
    let req = test::TestRequest::get()
        .uri(&format!("/offer/{}", created.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let body: Offer = test::read_body_json(resp).await;
    assert_eq!(body.id, created.id);
    assert_eq!(body.title, "Stage Data");
}

#[actix_web::test]
async fn test_get_offer_not_found() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let req = test::TestRequest::get()
        .uri(&format!("/offer/{}", uuid::Uuid::new_v4()))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_unavailable_offer_returns_404() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let payload = json!({
        "title": "Stage Inactif",
        "link": "http://example.com",
        "city": "Paris",
        "domain": "IT",
        "salary": 1000.0,
        "start_date": "2026-06-01",
        "end_date": "2026-12-01",
        "available": false
    });

    let req = test::TestRequest::post()
        .uri("/offer")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    let created: Offer = test::read_body_json(resp).await;

    // GET should return 404 because available=false
    let req = test::TestRequest::get()
        .uri(&format!("/offer/{}", created.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

// ─── GET /offer?domain= ───

#[actix_web::test]
async fn test_list_offers_by_domain() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    // Create two IT offers and one Life Science offer
    for (title, domain) in [
        ("Stage 1", "IT"),
        ("Stage 2", "IT"),
        ("Stage 3", "life science"),
    ] {
        let payload = json!({
            "title": title,
            "link": "http://example.com",
            "city": "Paris",
            "domain": domain,
            "salary": 1000.0,
            "start_date": "2026-06-01",
            "end_date": "2026-12-01",
            "available": true
        });
        let req = test::TestRequest::post()
            .uri("/offer")
            .set_json(&payload)
            .to_request();
        test::call_service(&app, req).await;
    }

    let req = test::TestRequest::get()
        .uri("/offer?domain=IT")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let body: Vec<Offer> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 2);
}

// ─── GET /offer?city= ───

#[actix_web::test]
async fn test_list_offers_by_city() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    for city in ["Paris", "Paris", "Lyon"] {
        let payload = json!({
            "title": "Stage",
            "link": "http://example.com",
            "city": city,
            "domain": "IT",
            "salary": 1000.0,
            "start_date": "2026-06-01",
            "end_date": "2026-12-01",
            "available": true
        });
        let req = test::TestRequest::post()
            .uri("/offer")
            .set_json(&payload)
            .to_request();
        test::call_service(&app, req).await;
    }

    let req = test::TestRequest::get()
        .uri("/offer?city=Paris")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let body: Vec<Offer> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 2);
}

// ─── PUT /offer/{id} ───

#[actix_web::test]
async fn test_update_offer() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let payload = json!({
        "title": "Stage Original",
        "link": "http://example.com",
        "city": "Paris",
        "domain": "IT",
        "salary": 1000.0,
        "start_date": "2026-06-01",
        "end_date": "2026-12-01",
        "available": true
    });
    let req = test::TestRequest::post()
        .uri("/offer")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    let created: Offer = test::read_body_json(resp).await;

    // Update the title
    let update_payload = json!({ "title": "Stage Modifié" });
    let req = test::TestRequest::put()
        .uri(&format!("/offer/{}", created.id))
        .set_json(&update_payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let body: Offer = test::read_body_json(resp).await;
    assert_eq!(body.title, "Stage Modifié");
    assert_eq!(body.city, "Paris"); // unchanged
}

// ─── DELETE /offer/{id} ───

#[actix_web::test]
async fn test_delete_offer() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let payload = json!({
        "title": "Stage à supprimer",
        "link": "http://example.com",
        "city": "Paris",
        "domain": "IT",
        "salary": 1000.0,
        "start_date": "2026-06-01",
        "end_date": "2026-12-01",
        "available": true
    });
    let req = test::TestRequest::post()
        .uri("/offer")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    let created: Offer = test::read_body_json(resp).await;

    // Delete
    let req = test::TestRequest::delete()
        .uri(&format!("/offer/{}", created.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 204);

    // Verify it's gone
    let req = test::TestRequest::get()
        .uri(&format!("/offer/{}", created.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_delete_offer_not_found() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let req = test::TestRequest::delete()
        .uri(&format!("/offer/{}", uuid::Uuid::new_v4()))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

// ─── GET /offer without query params ───

#[actix_web::test]
async fn test_list_offers_without_filter_returns_400() {
    let state = test_app_state();
    let app = test::init_service(test_app(state)).await;

    let req = test::TestRequest::get().uri("/offer").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}
