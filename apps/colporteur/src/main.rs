use lapin::{BasicProperties, Connection, ConnectionProperties, options::*};
use serde::Serialize;

#[derive(Serialize)]
struct NewsEvent {
    name: String,
    source: String,
    date: String,
    tags: Vec<String>,
    city: String,
    country: String,
}

fn sample_news() -> Vec<NewsEvent> {
    vec![
        NewsEvent {
            name: "Paris launches new metro line".into(),
            source: "Le Monde".into(),
            date: "2026-02-27".into(),
            tags: vec!["innovation".into(), "economy".into()],
            city: "Paris".into(),
            country: "France".into(),
        },
        NewsEvent {
            name: "Berlin startup ecosystem booming".into(),
            source: "TechCrunch".into(),
            date: "2026-02-26".into(),
            tags: vec!["innovation".into(), "economy".into()],
            city: "Berlin".into(),
            country: "Germany".into(),
        },
        NewsEvent {
            name: "Barcelona hosts international film festival".into(),
            source: "El País".into(),
            date: "2026-02-25".into(),
            tags: vec!["festival".into(), "culture".into(), "tourism".into()],
            city: "Barcelona".into(),
            country: "Spain".into(),
        },
        NewsEvent {
            name: "Crime rates decrease in London".into(),
            source: "BBC".into(),
            date: "2026-02-24".into(),
            tags: vec!["crime".into(), "politics".into()],
            city: "London".into(),
            country: "UK".into(),
        },
        NewsEvent {
            name: "Paris air quality improvement plan".into(),
            source: "France24".into(),
            date: "2026-02-23".into(),
            tags: vec!["pollution".into(), "health".into()],
            city: "Paris".into(),
            country: "France".into(),
        },
        NewsEvent {
            name: "New university campus opens in Berlin".into(),
            source: "DW".into(),
            date: "2026-02-22".into(),
            tags: vec!["education".into(), "innovation".into()],
            city: "Berlin".into(),
            country: "Germany".into(),
        },
        NewsEvent {
            name: "Barcelona marathon attracts 50,000 runners".into(),
            source: "La Vanguardia".into(),
            date: "2026-02-21".into(),
            tags: vec!["sports".into(), "tourism".into(), "health".into()],
            city: "Barcelona".into(),
            country: "Spain".into(),
        },
        NewsEvent {
            name: "London tech week announces keynote speakers".into(),
            source: "The Guardian".into(),
            date: "2026-02-20".into(),
            tags: vec!["innovation".into(), "education".into(), "economy".into()],
            city: "London".into(),
            country: "UK".into(),
        },
        NewsEvent {
            name: "Paris museum night event".into(),
            source: "Le Figaro".into(),
            date: "2026-02-19".into(),
            tags: vec!["festival".into(), "culture".into(), "tourism".into()],
            city: "Paris".into(),
            country: "France".into(),
        },
        NewsEvent {
            name: "Berlin green energy initiative".into(),
            source: "Spiegel".into(),
            date: "2026-02-18".into(),
            tags: vec!["economy".into(), "innovation".into(), "health".into()],
            city: "Berlin".into(),
            country: "Germany".into(),
        },
    ]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let rabbitmq_url = std::env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string());

    println!("Colporteur — Connecting to RabbitMQ at {}", rabbitmq_url);

    let conn = Connection::connect(&rabbitmq_url, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    // Declare exchange
    channel
        .exchange_declare(
            "zukmove.events",
            lapin::ExchangeKind::Topic,
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            Default::default(),
        )
        .await?;

    let news_items = sample_news();
    println!("Injecting {} news items via RabbitMQ...", news_items.len());

    for (i, news) in news_items.iter().enumerate() {
        let payload = serde_json::to_vec(news)?;

        channel
            .basic_publish(
                "zukmove.events",
                "news.created",
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default()
                    .with_content_type("application/json".into())
                    .with_delivery_mode(2), // persistent
            )
            .await?
            .await?;

        println!(
            "  [{}/{}] {} (city: {}, tags: {:?})",
            i + 1,
            news_items.len(),
            news.name,
            news.city,
            news.tags,
        );
    }

    println!("\nAll news injected successfully via RabbitMQ!");
    Ok(())
}
