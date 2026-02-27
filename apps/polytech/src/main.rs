mod adapters;
mod routes;

#[cfg(test)]
mod tests;

mod proto {
    tonic::include_proto!("mi8");
}

use std::sync::Mutex;

use actix_web::{App, HttpServer, web};
use sqlx::PgPool;

use adapters::grpc_news_client::GrpcNewsClient;
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

    let mi8_url = std::env::var("MI8_URL").unwrap_or_else(|_| "http://localhost:50051".to_string());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    // Connect to PostgreSQL
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    // Run migrations (one statement at a time — sqlx doesn't support multi-statement queries)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS students (
            id UUID PRIMARY KEY,
            firstname VARCHAR(255) NOT NULL,
            name VARCHAR(255) NOT NULL,
            domain VARCHAR(100) NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create students table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS internships (
            id UUID PRIMARY KEY,
            student_id UUID NOT NULL REFERENCES students(id),
            offer_id UUID NOT NULL,
            status VARCHAR(20) NOT NULL,
            message TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create internships table");

    log::info!("Connected to PostgreSQL and migrations applied");

    // Build adapters
    let student_repo = PostgresStudentRepository::new(pool.clone());
    let internship_repo = PostgresInternshipRepository::new(pool.clone());
    let offer_client = HttpOfferClient::new(erasmumu_url);

    let student_repo_for_service = PostgresStudentRepository::new(pool.clone());

    let internship_service = InternshipService::new(
        Box::new(student_repo_for_service),
        Box::new(internship_repo),
        Box::new(offer_client),
    );

    let state = web::Data::new(AppState {
        student_repo: Box::new(student_repo),
        internship_service,
    });

    // MI8 gRPC client
    let grpc_client = GrpcNewsClient::new(mi8_url)
        .await
        .expect("Failed to connect to MI8 gRPC service");
    let grpc_client_data = web::Data::new(Mutex::new(grpc_client));

    log::info!("Starting Polytech service on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(grpc_client_data.clone())
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
            // News routes (proxy to MI8 via gRPC)
            .route("/news", web::get().to(routes::news::get_news))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
