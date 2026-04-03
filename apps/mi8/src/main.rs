mod adapters;
mod service;

use std::collections::HashMap;
use std::sync::Arc;

use lapin::{Connection, ConnectionProperties, options::*};
use tokio::sync::Mutex;
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("mi8");
}

use adapters::redis_city_score_repo::RedisCityScoreRepository;
use adapters::redis_news_repo::RedisNewsRepository;
use proto::mi8_service_server::Mi8ServiceServer;
use service::Mi8ServiceImpl;
use zukmove_core::domain::entities::news::News;
use zukmove_core::domain::ports::{CityScoreRepository, NewsRepository};

#[derive(serde::Deserialize)]
struct NewsEvent {
    name: String,
    source: String,
    date: String,
    tags: Vec<String>,
    city: String,
    country: String,
}

#[derive(serde::Deserialize)]
struct OfferEvent {
    city: String,
    domain: String,
    #[serde(default)]
    start_date: String,
}

/// City stats stored in Redis
#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
struct CityStatsData {
    city: String,
    total_offers: i32,
    offers_by_domain: HashMap<String, i32>,
    last_offer_date: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let port = std::env::var("MI8_PORT").unwrap_or_else(|_| "50051".to_string());
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let rabbitmq_url = std::env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;

    let news_repo = Arc::new(
        RedisNewsRepository::new(&redis_url).expect("Failed to create Redis news repository"),
    );
    let city_score_repo = Arc::new(
        RedisCityScoreRepository::new(&redis_url)
            .expect("Failed to create Redis city score repository"),
    );

    // Redis client for city stats
    let redis_client = redis::Client::open(redis_url.as_str())?;

    let service = Mi8ServiceImpl::new(
        Box::new(RedisNewsRepository::new(&redis_url).unwrap()),
        Box::new(RedisCityScoreRepository::new(&redis_url).unwrap()),
        redis_client.clone(),
    );

    // Spawn RabbitMQ consumers
    let news_repo_clone = news_repo.clone();
    let city_score_repo_clone = city_score_repo.clone();
    let redis_client_clone = redis_client.clone();
    let rabbitmq_url_clone = rabbitmq_url.clone();

    tokio::spawn(async move {
        // Retry loop — wait for RabbitMQ to become available
        loop {
            match run_rabbitmq_consumers(
                &rabbitmq_url_clone,
                news_repo_clone.clone(),
                city_score_repo_clone.clone(),
                redis_client_clone.clone(),
            )
            .await
            {
                Ok(()) => break,
                Err(e) => {
                    log::warn!("RabbitMQ not ready, retrying in 5s: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }
    });

    log::info!(
        "MI8 gRPC server listening on {} (RabbitMQ: {})",
        addr,
        rabbitmq_url
    );

    Server::builder()
        .add_service(Mi8ServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

async fn run_rabbitmq_consumers(
    rabbitmq_url: &str,
    news_repo: Arc<dyn NewsRepository>,
    city_score_repo: Arc<dyn CityScoreRepository>,
    redis_client: redis::Client,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let conn = Connection::connect(rabbitmq_url, ConnectionProperties::default()).await?;

    // --- news.created consumer ---
    let news_channel = conn.create_channel().await?;
    news_channel
        .exchange_declare(
            "zukmove.events",
            lapin::ExchangeKind::Topic,
            ExchangeDeclareOptions { durable: true, ..Default::default() },
            Default::default(),
        )
        .await?;
    news_channel
        .queue_declare("mi8.news.created", QueueDeclareOptions { durable: true, ..Default::default() }, Default::default())
        .await?;
    news_channel
        .queue_bind("mi8.news.created", "zukmove.events", "news.created", QueueBindOptions::default(), Default::default())
        .await?;

    let news_consumer = news_channel
        .basic_consume("mi8.news.created", "mi8-news", BasicConsumeOptions::default(), Default::default())
        .await?;

    let news_repo_c = news_repo.clone();
    let city_score_repo_c = city_score_repo.clone();

    tokio::spawn(async move {
        use futures_lite::StreamExt;
        let mut consumer = news_consumer;
        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    if let Err(e) = process_news_event(&delivery.data, &news_repo_c, &city_score_repo_c).await {
                        log::error!("Failed to process news event: {}", e);
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
        .queue_declare("mi8.offer.created", QueueDeclareOptions { durable: true, ..Default::default() }, Default::default())
        .await?;
    offer_channel
        .queue_bind("mi8.offer.created", "zukmove.events", "offer.created", QueueBindOptions::default(), Default::default())
        .await?;

    let offer_consumer = offer_channel
        .basic_consume("mi8.offer.created", "mi8-offer", BasicConsumeOptions::default(), Default::default())
        .await?;

    let redis_client_c = redis_client.clone();

    tokio::spawn(async move {
        use futures_lite::StreamExt;
        let mut consumer = offer_consumer;
        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    if let Err(e) = process_offer_event(&delivery.data, &redis_client_c).await {
                        log::error!("Failed to process offer event: {}", e);
                    }
                    let _ = delivery.ack(BasicAckOptions::default()).await;
                }
                Err(e) => log::error!("Consumer error: {}", e),
            }
        }
    });

    log::info!("MI8 RabbitMQ consumers started (news.created, offer.created)");
    Ok(())
}

async fn process_news_event(
    data: &[u8],
    news_repo: &Arc<dyn NewsRepository>,
    city_score_repo: &Arc<dyn CityScoreRepository>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let event: NewsEvent = serde_json::from_slice(data)?;

    let news = News {
        id: uuid::Uuid::new_v4().to_string(),
        name: event.name.clone(),
        source: event.source,
        date: if event.date.is_empty() {
            chrono::Utc::now().format("%Y-%m-%d").to_string()
        } else {
            event.date
        },
        tags: event.tags.clone(),
        city: event.city.clone(),
        country: event.country.clone(),
    };

    news_repo.save(&news).await?;

    if !event.tags.is_empty() {
        let mut score = city_score_repo
            .get_or_create(&event.city, &event.country)
            .await?;
        score.apply_tags(&event.tags);
        city_score_repo.save(&score).await?;
    }

    log::info!("Processed news event: {} (city: {})", event.name, event.city);
    Ok(())
}

async fn process_offer_event(
    data: &[u8],
    redis_client: &redis::Client,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let event: OfferEvent = serde_json::from_slice(data)?;

    let mut conn = redis_client.get_multiplexed_async_connection().await?;
    let key = format!("citystats:data:{}", event.city.to_lowercase());

    let existing: Option<String> = redis::AsyncCommands::get(&mut conn, &key).await?;

    let mut stats: CityStatsData = if let Some(json) = existing {
        serde_json::from_str(&json)?
    } else {
        CityStatsData {
            city: event.city.clone(),
            ..Default::default()
        }
    };

    stats.total_offers += 1;
    *stats.offers_by_domain.entry(event.domain.clone()).or_insert(0) += 1;
    if !event.start_date.is_empty() {
        stats.last_offer_date = event.start_date;
    } else {
        stats.last_offer_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    }

    let json = serde_json::to_string(&stats)?;
    let _: () = redis::AsyncCommands::set(&mut conn, &key, &json).await?;

    log::info!(
        "Updated city stats for {}: {} total offers",
        event.city,
        stats.total_offers
    );
    Ok(())
}
