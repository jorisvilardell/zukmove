mod adapters;
mod routes;

#[cfg(test)]
mod tests;

use actix_web::{App, HttpServer, web};
use mongodb::Client;

use adapters::mongo_offer::MongoOfferRepository;
use zukmove_core::domain::ports::OfferRepository;

pub struct AppState {
    pub offer_repo: Box<dyn OfferRepository>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mongo_url =
        std::env::var("MONGO_URL").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let mongo_db = std::env::var("MONGO_DB").unwrap_or_else(|_| "erasmumu".to_string());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse()
        .expect("PORT must be a number");

    // Connect to MongoDB
    let client = Client::with_uri_str(&mongo_url)
        .await
        .expect("Failed to connect to MongoDB");

    log::info!("Connected to MongoDB");

    let offer_repo = MongoOfferRepository::new(&client, &mongo_db);

    let state = web::Data::new(AppState {
        offer_repo: Box::new(offer_repo),
    });

    log::info!("Starting Erasmumu service on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
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
