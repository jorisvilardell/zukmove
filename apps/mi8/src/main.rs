mod adapters;
mod service;

use tonic::transport::Server;

mod proto {
    tonic::include_proto!("mi8");
}

use adapters::array_news_repo::ArrayNewsRepository;
use adapters::in_memory_city_score_repo::InMemoryCityScoreRepository;
use proto::mi8_service_server::Mi8ServiceServer;
use service::Mi8ServiceImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let port = std::env::var("MI8_PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;

    let news_repo = Box::new(ArrayNewsRepository::new());
    let city_score_repo = Box::new(InMemoryCityScoreRepository::new());

    let service = Mi8ServiceImpl::new(news_repo, city_score_repo);

    log::info!("MI8 gRPC server listening on {}", addr);

    Server::builder()
        .add_service(Mi8ServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
