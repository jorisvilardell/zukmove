mod adapters;
mod routes;

use actix_web::{App, HttpServer, web};
use sqlx::PgPool;

use adapters::http_offer_client::HttpOfferClient;
use adapters::postgres_internship::PostgresInternshipRepository;
use adapters::postgres_student::PostgresStudentRepository;
use zukmove_core::app::internship_service::InternshipService;
use zukmove_core::domain::ports::StudentRepository;

pub struct AppState {
    pub student_repo: Box<dyn StudentRepository>,
    pub internship_service: InternshipService,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://polytech:polytech@localhost:5432/polytech".to_string());

    let erasmumu_url =
        std::env::var("ERASMUMU_URL").unwrap_or_else(|_| "http://localhost:8081".to_string());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    // Connect to PostgreSQL
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    // Run migrations
    sqlx::query(include_str!("../migrations/001_init.sql"))
        .execute(&pool)
        .await
        .expect("Failed to run migrations");

    log::info!("Connected to PostgreSQL and migrations applied");

    // Build adapters
    let student_repo = PostgresStudentRepository::new(pool.clone());
    let internship_repo = PostgresInternshipRepository::new(pool.clone());
    let offer_client = HttpOfferClient::new(erasmumu_url);

    // One copy of student_repo for direct use in routes, another for internship service
    let student_repo_for_service = PostgresStudentRepository::new(pool.clone());

    // Build use case services
    let internship_service = InternshipService::new(
        Box::new(student_repo_for_service),
        Box::new(internship_repo),
        Box::new(offer_client),
    );

    let state = web::Data::new(AppState {
        student_repo: Box::new(student_repo),
        internship_service,
    });

    log::info!("Starting Polytech service on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            // Student routes
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
            // Internship routes
            .route(
                "/internship",
                web::post().to(routes::internship::create_internship),
            )
            .route(
                "/internship/{id}",
                web::get().to(routes::internship::get_internship),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
