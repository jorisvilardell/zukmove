mod proto {
    tonic::include_proto!("mi8");
}

use proto::mi8_service_client::Mi8ServiceClient;
use proto::CreateNewsRequest;

/// Données fictives à injecter dans MI8.
fn sample_news() -> Vec<CreateNewsRequest> {
    vec![
        CreateNewsRequest {
            name: "Paris launches new metro line".to_string(),
            source: "Le Monde".to_string(),
            date: "2026-02-27".to_string(),
            tags: vec!["innovation".to_string(), "economy".to_string()],
            city: "Paris".to_string(),
            country: "France".to_string(),
        },
        CreateNewsRequest {
            name: "Berlin startup ecosystem booming".to_string(),
            source: "TechCrunch".to_string(),
            date: "2026-02-26".to_string(),
            tags: vec!["innovation".to_string(), "economy".to_string()],
            city: "Berlin".to_string(),
            country: "Germany".to_string(),
        },
        CreateNewsRequest {
            name: "Barcelona hosts international film festival".to_string(),
            source: "El País".to_string(),
            date: "2026-02-25".to_string(),
            tags: vec!["festival".to_string(), "culture".to_string(), "tourism".to_string()],
            city: "Barcelona".to_string(),
            country: "Spain".to_string(),
        },
        CreateNewsRequest {
            name: "Crime rates decrease in London".to_string(),
            source: "BBC".to_string(),
            date: "2026-02-24".to_string(),
            tags: vec!["crime".to_string(), "politics".to_string()],
            city: "London".to_string(),
            country: "UK".to_string(),
        },
        CreateNewsRequest {
            name: "Paris air quality improvement plan".to_string(),
            source: "France24".to_string(),
            date: "2026-02-23".to_string(),
            tags: vec!["pollution".to_string(), "health".to_string()],
            city: "Paris".to_string(),
            country: "France".to_string(),
        },
        CreateNewsRequest {
            name: "New university campus opens in Berlin".to_string(),
            source: "DW".to_string(),
            date: "2026-02-22".to_string(),
            tags: vec!["education".to_string(), "innovation".to_string()],
            city: "Berlin".to_string(),
            country: "Germany".to_string(),
        },
        CreateNewsRequest {
            name: "Barcelona marathon attracts 50,000 runners".to_string(),
            source: "La Vanguardia".to_string(),
            date: "2026-02-21".to_string(),
            tags: vec!["sports".to_string(), "tourism".to_string(), "health".to_string()],
            city: "Barcelona".to_string(),
            country: "Spain".to_string(),
        },
        CreateNewsRequest {
            name: "London tech week announces keynote speakers".to_string(),
            source: "The Guardian".to_string(),
            date: "2026-02-20".to_string(),
            tags: vec!["innovation".to_string(), "education".to_string(), "economy".to_string()],
            city: "London".to_string(),
            country: "UK".to_string(),
        },
        CreateNewsRequest {
            name: "Paris museum night event".to_string(),
            source: "Le Figaro".to_string(),
            date: "2026-02-19".to_string(),
            tags: vec!["festival".to_string(), "culture".to_string(), "tourism".to_string()],
            city: "Paris".to_string(),
            country: "France".to_string(),
        },
        CreateNewsRequest {
            name: "Berlin green energy initiative".to_string(),
            source: "Spiegel".to_string(),
            date: "2026-02-18".to_string(),
            tags: vec!["economy".to_string(), "innovation".to_string(), "health".to_string()],
            city: "Berlin".to_string(),
            country: "Germany".to_string(),
        },
    ]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mi8_url =
        std::env::var("MI8_URL").unwrap_or_else(|_| "http://localhost:50051".to_string());

    println!("🗞️  Colporteur — Connecting to MI8 at {}", mi8_url);

    let mut client = Mi8ServiceClient::connect(mi8_url).await?;

    let news_items = sample_news();
    println!("📰 Injecting {} news items...", news_items.len());

    for (i, news) in news_items.into_iter().enumerate() {
        let city = news.city.clone();
        let name = news.name.clone();
        let tags = news.tags.clone();

        let response = client
            .create_news(tonic::Request::new(news))
            .await?;

        let created = response.into_inner();
        println!(
            "  [{}/10] ✅ {} (city: {}, tags: {:?}, id: {})",
            i + 1,
            name,
            city,
            tags,
            created.id,
        );
    }

    println!("\n🎉 All news injected successfully!");
    println!("\nYou can now query Polytech:");
    println!("  curl http://localhost:8080/news?limit=5");
    println!("  curl http://localhost:8080/news?city=Paris&limit=3");

    Ok(())
}
