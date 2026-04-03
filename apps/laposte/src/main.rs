use std::collections::HashMap;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, web};
use lapin::{Connection, ConnectionProperties, options::*};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

// ─── Data Model ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    #[serde(rename = "studentId")]
    pub student_id: String,
    pub domain: String,
    pub channel: String,
    pub contact: String,
    pub enabled: bool,
}

type SubscriberStore = Arc<Mutex<HashMap<String, Subscriber>>>;

// ─── Event structures ───

#[derive(Deserialize)]
struct StudentRegisteredEvent {
    #[serde(rename = "studentId")]
    student_id: String,
    #[allow(dead_code)]
    name: String,
    domain: String,
}

#[derive(Deserialize)]
struct OfferCreatedEvent {
    #[allow(dead_code)]
    id: String,
    title: String,
    city: String,
    domain: String,
}

// ─── REST Handlers ───

async fn get_subscriber(
    store: web::Data<SubscriberStore>,
    path: web::Path<String>,
) -> HttpResponse {
    let student_id = path.into_inner();
    let store = store.lock().await;
    match store.get(&student_id) {
        Some(sub) => HttpResponse::Ok().json(sub),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Subscriber {} not found", student_id)
        })),
    }
}

#[derive(Deserialize)]
struct UpdateSubscriberRequest {
    channel: Option<String>,
    contact: Option<String>,
    enabled: Option<bool>,
}

async fn update_subscriber(
    store: web::Data<SubscriberStore>,
    path: web::Path<String>,
    body: web::Json<UpdateSubscriberRequest>,
) -> HttpResponse {
    let student_id = path.into_inner();
    let mut store = store.lock().await;

    match store.get_mut(&student_id) {
        Some(sub) => {
            if let Some(ref channel) = body.channel {
                sub.channel = channel.clone();
            }
            if let Some(ref contact) = body.contact {
                sub.contact = contact.clone();
            }
            if let Some(enabled) = body.enabled {
                sub.enabled = enabled;
            }
            HttpResponse::Ok().json(sub.clone())
        }
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Subscriber {} not found", student_id)
        })),
    }
}

async fn delete_subscriber(
    store: web::Data<SubscriberStore>,
    path: web::Path<String>,
) -> HttpResponse {
    let student_id = path.into_inner();
    let mut store = store.lock().await;

    if store.remove(&student_id).is_some() {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Subscriber {} not found", student_id)
        }))
    }
}

// ─── RabbitMQ Consumers ───

async fn run_consumers(
    rabbitmq_url: &str,
    store: SubscriberStore,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let conn = Connection::connect(rabbitmq_url, ConnectionProperties::default()).await?;

    // Declare exchange
    let setup_channel = conn.create_channel().await?;
    setup_channel
        .exchange_declare(
            "zukmove.events",
            lapin::ExchangeKind::Topic,
            ExchangeDeclareOptions { durable: true, ..Default::default() },
            Default::default(),
        )
        .await?;

    // --- student.registered consumer ---
    let student_channel = conn.create_channel().await?;
    student_channel
        .queue_declare("laposte.student.registered", QueueDeclareOptions { durable: true, ..Default::default() }, Default::default())
        .await?;
    student_channel
        .queue_bind("laposte.student.registered", "zukmove.events", "student.registered", QueueBindOptions::default(), Default::default())
        .await?;

    let student_consumer = student_channel
        .basic_consume("laposte.student.registered", "laposte-student", BasicConsumeOptions::default(), Default::default())
        .await?;

    let store_clone = store.clone();
    tokio::spawn(async move {
        use futures_lite::StreamExt;
        let mut consumer = student_consumer;
        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    if let Ok(event) = serde_json::from_slice::<StudentRegisteredEvent>(&delivery.data) {
                        let sub = Subscriber {
                            student_id: event.student_id.clone(),
                            domain: event.domain,
                            channel: "email".to_string(),
                            contact: String::new(),
                            enabled: true,
                        };
                        store_clone.lock().await.insert(event.student_id.clone(), sub);
                        log::info!("Registered subscriber: {}", event.student_id);
                    }
                    let _ = delivery.ack(BasicAckOptions::default()).await;
                }
                Err(e) => log::error!("Consumer error: {}", e),
            }
        }
    });

    // --- offer.created consumer ---
    let offer_channel = conn.create_channel().await?;
    offer_channel
        .queue_declare("laposte.offer.created", QueueDeclareOptions { durable: true, ..Default::default() }, Default::default())
        .await?;
    offer_channel
        .queue_bind("laposte.offer.created", "zukmove.events", "offer.created", QueueBindOptions::default(), Default::default())
        .await?;

    let offer_consumer = offer_channel
        .basic_consume("laposte.offer.created", "laposte-offer", BasicConsumeOptions::default(), Default::default())
        .await?;

    let store_clone2 = store.clone();
    tokio::spawn(async move {
        use futures_lite::StreamExt;
        let mut consumer = offer_consumer;
        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    if let Ok(event) = serde_json::from_slice::<OfferCreatedEvent>(&delivery.data) {
                        let store = store_clone2.lock().await;
                        for (id, sub) in store.iter() {
                            if sub.enabled && sub.domain.to_lowercase() == event.domain.to_lowercase() {
                                log::info!(
                                    "[MOCK ALERT] Notifying {} via {} ({}): New offer '{}' in {}",
                                    id,
                                    sub.channel,
                                    if sub.contact.is_empty() { "no contact" } else { &sub.contact },
                                    event.title,
                                    event.city,
                                );
                            }
                        }
                    }
                    let _ = delivery.ack(BasicAckOptions::default()).await;
                }
                Err(e) => log::error!("Consumer error: {}", e),
            }
        }
    });

    log::info!("La Poste: RabbitMQ consumers started (student.registered, offer.created)");
    Ok(())
}

// ─── Main ───

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8083".to_string())
        .parse()
        .expect("PORT must be a number");

    let rabbitmq_url = std::env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string());

    let store: SubscriberStore = Arc::new(Mutex::new(HashMap::new()));

    // Start RabbitMQ consumers
    let store_clone = store.clone();
    let rabbitmq_url_clone = rabbitmq_url.clone();
    tokio::spawn(async move {
        loop {
            match run_consumers(&rabbitmq_url_clone, store_clone.clone()).await {
                Ok(()) => break,
                Err(e) => {
                    log::warn!("RabbitMQ not ready, retrying in 5s: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }
    });

    let store_data = web::Data::new(store);

    log::info!("Starting La Poste service on port {}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(store_data.clone())
            .route(
                "/subscribers/{studentId}",
                web::get().to(get_subscriber),
            )
            .route(
                "/subscribers/{studentId}",
                web::put().to(update_subscriber),
            )
            .route(
                "/subscribers/{studentId}",
                web::delete().to(delete_subscriber),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
