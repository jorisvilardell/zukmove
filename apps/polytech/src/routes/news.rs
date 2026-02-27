use actix_web::{HttpResponse, web};
use std::sync::Mutex;

use crate::adapters::grpc_news_client::GrpcNewsClient;

#[derive(serde::Deserialize)]
pub struct NewsQuery {
    pub limit: Option<i32>,
    pub city: Option<String>,
}

/// GET /news?limit=N&city=Paris
pub async fn get_news(
    grpc_client: web::Data<Mutex<GrpcNewsClient>>,
    query: web::Query<NewsQuery>,
) -> HttpResponse {
    let limit = query.limit.unwrap_or(10);
    let mut client = grpc_client.lock().unwrap();

    let result = if let Some(ref city) = query.city {
        client.get_latest_news_in_city(city.clone(), limit).await
    } else {
        client.get_latest_news(limit).await
    };

    match result {
        Ok(news) => HttpResponse::Ok().json(news),
        Err(e) => {
            log::error!("Failed to fetch news from MI8: {}", e);
            HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": format!("MI8 service unavailable: {}", e)
            }))
        }
    }
}
