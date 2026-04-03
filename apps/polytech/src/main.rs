mod adapters;
mod routes;

#[cfg(test)]
mod tests;

mod proto {
    tonic::include_proto!("mi8");
}

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use adapters::grpc_news_client::GrpcNewsClient;
use adapters::http_offer_client::HttpOfferClient;
use adapters::postgres_internship::PostgresInternshipRepository;
use adapters::postgres_notification::PostgresNotificationRepository;
use adapters::postgres_student::PostgresStudentRepository;
use zukmove_core::app::internship_service::InternshipService;
use zukmove_core::domain::entities::gateway::AggregatedOffer;
use zukmove_core::domain::entities::internship::{
    CreateInternshipRequest, Internship, InternshipStatus,
};
use zukmove_core::domain::entities::notification::Notification;
use zukmove_core::domain::entities::student::{
    CreateStudentRequest, Student, UpdateStudentRequest,
};
use zukmove_core::domain::ports::{NotificationRepository, StudentRepository};

pub struct AppState {
    pub student_repo: Box<dyn StudentRepository>,
    pub internship_service: InternshipService,
    pub notification_repo: Box<dyn NotificationRepository>,
    pub rabbitmq_channel: Option<lapin::Channel>,
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Polytech API",
        version = "1.0.0",
        description = "Service de gestion des étudiants et stages"
    ),
    paths(
        routes::student::create_student,
        routes::student::list_students,
        routes::student::get_student,
        routes::student::update_student,
        routes::student::delete_student,
        routes::student::get_recommended_offers,
        routes::internship::create_internship,
        routes::internship::get_internship,
        routes::offer::get_offers,
    ),
    components(schemas(
        Student,
        CreateStudentRequest,
        UpdateStudentRequest,
        Internship,
        InternshipStatus,
        CreateInternshipRequest,
        AggregatedOffer,
        Notification,
    ))
)]
struct ApiDoc;

#[derive(serde::Deserialize)]
struct OfferEvent {
    id: uuid::Uuid,
    title: String,
    city: String,
    domain: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://polytech:polytech@localhost:5432/polytech".to_string());

    let erasmumu_url =
        std::env::var("ERASMUMU_URL").unwrap_or_else(|_| "http://localhost:8081".to_string());

    let mi8_url = std::env::var("MI8_URL").unwrap_or_else(|_| "http://localhost:50051".to_string());

    let rabbitmq_url = std::env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    // Connect to PostgreSQL
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    // Run migrations
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

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS notifications (
            id UUID PRIMARY KEY,
            student_id UUID NOT NULL REFERENCES students(id),
            type VARCHAR(50) NOT NULL,
            offer_id UUID NOT NULL,
            message TEXT NOT NULL,
            read BOOLEAN NOT NULL DEFAULT false
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create notifications table");

    log::info!("Connected to PostgreSQL and migrations applied");

    // Build adapters
    let student_repo = PostgresStudentRepository::new(pool.clone());
    let internship_repo = PostgresInternshipRepository::new(pool.clone());
    let offer_client = HttpOfferClient::new(erasmumu_url);
    let notification_repo = PostgresNotificationRepository::new(pool.clone());

    let student_repo_for_service = PostgresStudentRepository::new(pool.clone());

    // MI8 gRPC client
    let grpc_client = GrpcNewsClient::new(mi8_url)
        .await
        .expect("Failed to connect to MI8 gRPC service");
    let grpc_client_data =
        web::Data::new(std::sync::Mutex::new(grpc_client.clone()));

    let internship_service = InternshipService::new(
        Box::new(student_repo_for_service),
        Box::new(internship_repo),
        Box::new(offer_client),
        Arc::new(grpc_client.clone()),
    );

    // Connect to RabbitMQ
    let rabbitmq_channel = match lapin::Connection::connect(
        &rabbitmq_url,
        lapin::ConnectionProperties::default(),
    )
    .await
    {
        Ok(conn) => {
            let channel = conn.create_channel().await.ok();
            if let Some(ref ch) = channel {
                let _ = ch
                    .exchange_declare(
                        "zukmove.events",
                        lapin::ExchangeKind::Topic,
                        lapin::options::ExchangeDeclareOptions {
                            durable: true,
                            ..Default::default()
                        },
                        Default::default(),
                    )
                    .await;
            }
            log::info!("Connected to RabbitMQ");

            // Start offer.created consumer
            let consumer_rabbitmq_url = rabbitmq_url.clone();
            let consumer_pool = pool.clone();
            tokio::spawn(async move {
                loop {
                    match lapin::Connection::connect(
                        &consumer_rabbitmq_url,
                        lapin::ConnectionProperties::default(),
                    )
                    .await
                    {
                        Ok(conn) => {
                            if let Err(e) = run_offer_created_consumer(conn, consumer_pool.clone()).await {
                                log::warn!("offer.created consumer disconnected: {}", e);
                            }
                        }
                        Err(e) => {
                            log::warn!("RabbitMQ consumer connect failed, retrying in 5s: {}", e);
                        }
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            });

            channel
        }
        Err(e) => {
            log::warn!(
                "Failed to connect to RabbitMQ: {} — continuing without events",
                e
            );
            None
        }
    };

    let state = web::Data::new(AppState {
        student_repo: Box::new(student_repo),
        internship_service,
        notification_repo: Box::new(notification_repo),
        rabbitmq_channel,
    });

    log::info!(
        "Starting Polytech service on port {} — Swagger UI: http://localhost:{}/swagger-ui/",
        port,
        port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(state.clone())
            .app_data(grpc_client_data.clone())
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
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
            .route(
                "/student/{id}/recommended-offers",
                web::get().to(routes::student::get_recommended_offers),
            )
            // Notification routes
            .route(
                "/students/{id}/notifications",
                web::get().to(routes::notification::get_notifications),
            )
            .route(
                "/notifications/{id}/read",
                web::put().to(routes::notification::mark_as_read),
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
            // Configured Offer route
            .route("/offer", web::get().to(routes::offer::get_offers))
            // News routes (proxy to MI8 via gRPC)
            .route("/news", web::get().to(routes::news::get_news))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn run_offer_created_consumer(
    conn: lapin::Connection,
    pool: PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let channel = conn.create_channel().await?;

    channel
        .exchange_declare(
            "zukmove.events",
            lapin::ExchangeKind::Topic,
            lapin::options::ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            Default::default(),
        )
        .await?;

    channel
        .queue_declare(
            "polytech.offer.created",
            lapin::options::QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            Default::default(),
        )
        .await?;

    channel
        .queue_bind(
            "polytech.offer.created",
            "zukmove.events",
            "offer.created",
            lapin::options::QueueBindOptions::default(),
            Default::default(),
        )
        .await?;

    let consumer = channel
        .basic_consume(
            "polytech.offer.created",
            "polytech-offer",
            lapin::options::BasicConsumeOptions::default(),
            Default::default(),
        )
        .await?;

    log::info!("Polytech: offer.created consumer started");

    use futures_lite::StreamExt;
    let mut consumer = consumer;
    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(delivery) => {
                if let Err(e) = handle_offer_created(&delivery.data, &pool).await {
                    log::error!("Failed to handle offer.created: {}", e);
                }
                let _ = delivery
                    .ack(lapin::options::BasicAckOptions::default())
                    .await;
            }
            Err(e) => log::error!("Consumer error: {}", e),
        }
    }

    Ok(())
}

async fn handle_offer_created(
    data: &[u8],
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let event: OfferEvent = serde_json::from_slice(data)?;

    // Find all students with matching domain
    let students: Vec<(uuid::Uuid,)> =
        sqlx::query_as("SELECT id FROM students WHERE LOWER(domain) = LOWER($1)")
            .bind(&event.domain)
            .fetch_all(pool)
            .await?;

    for (student_id,) in students {
        let notif_id = uuid::Uuid::new_v4();
        let message = format!(
            "New offer: {} in {} (domain: {})",
            event.title, event.city, event.domain
        );

        sqlx::query(
            "INSERT INTO notifications (id, student_id, type, offer_id, message, read) VALUES ($1, $2, 'new_offer', $3, $4, false)",
        )
        .bind(notif_id)
        .bind(student_id)
        .bind(event.id)
        .bind(&message)
        .execute(pool)
        .await?;

        log::info!(
            "Created notification for student {} about offer {}",
            student_id,
            event.id
        );
    }

    Ok(())
}
