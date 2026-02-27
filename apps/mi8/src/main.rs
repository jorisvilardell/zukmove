mod adapters;
mod service;

use tonic::transport::Server;

mod proto {
    tonic::include_proto!("mi8");
}

use adapters::redis_city_score_repo::RedisCityScoreRepository;
use adapters::redis_news_repo::RedisNewsRepository;
use proto::mi8_service_server::Mi8ServiceServer;
use service::Mi8ServiceImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let port = std::env::var("MI8_PORT").unwrap_or_else(|_| "50051".to_string());
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;

    let news_repo = Box::new(
        RedisNewsRepository::new(&redis_url).expect("Failed to create Redis news repository"),
    );
    let city_score_repo = Box::new(
        RedisCityScoreRepository::new(&redis_url)
            .expect("Failed to create Redis city score repository"),
    );

    let service = Mi8ServiceImpl::new(news_repo, city_score_repo);

    log::info!(
        "MI8 gRPC server listening on {} (Redis: {})",
        addr,
        redis_url
    );

    Server::builder()
        .add_service(Mi8ServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
