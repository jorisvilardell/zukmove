mod adapters;
mod routes;

#[cfg(test)]
mod tests;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use mongodb::Client;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use adapters::mongo_offer::MongoOfferRepository;
use zukmove_core::domain::entities::offer::{CreateOfferRequest, Offer, UpdateOfferRequest};
use zukmove_core::domain::ports::OfferRepository;

pub struct AppState {
    pub offer_repo: Box<dyn OfferRepository>,
    pub rabbitmq_channel: Option<lapin::Channel>,
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Erasmumu API",
        version = "1.0.0",
        description = "Service de gestion des offres de stage"
    ),
    paths(
        routes::offer::create_offer,
        routes::offer::list_offers,
        routes::offer::get_offer,
        routes::offer::update_offer,
        routes::offer::delete_offer,
    ),
    components(schemas(Offer, CreateOfferRequest, UpdateOfferRequest,))
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let mongo_url =
        std::env::var("MONGO_URL").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let mongo_db = std::env::var("MONGO_DB").unwrap_or_else(|_| "erasmumu".to_string());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse()
        .expect("PORT must be a number");

    let rabbitmq_url = std::env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string());

    // Connect to MongoDB
    let client = Client::with_uri_str(&mongo_url)
        .await
        .expect("Failed to connect to MongoDB");

    log::info!("Connected to MongoDB");

    let offer_repo = MongoOfferRepository::new(&client, &mongo_db);

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
            channel
        }
        Err(e) => {
            log::warn!("Failed to connect to RabbitMQ: {} — continuing without events", e);
            None
        }
    };

    let state = web::Data::new(AppState {
        offer_repo: Box::new(offer_repo),
        rabbitmq_channel,
    });

    log::info!(
        "Starting Erasmumu service on port {} — Swagger UI: http://localhost:{}/swagger-ui/",
        port,
        port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(state.clone())
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            // Offer routes
            .route("/offer", web::post().to(routes::offer::create_offer))
            .route("/offer", web::get().to(routes::offer::list_offers))
            .route("/offer/{id}", web::get().to(routes::offer::get_offer))
            .route("/offer/{id}", web::put().to(routes::offer::update_offer))
            .route("/offer/{id}", web::delete().to(routes::offer::delete_offer))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
