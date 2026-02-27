use tonic::transport::Channel;

use crate::proto::mi8_service_client::Mi8ServiceClient;
use crate::proto::{GetLatestNewsInCityRequest, GetLatestNewsRequest};

/// Client gRPC pour appeler le service MI8.
pub struct GrpcNewsClient {
    client: Mi8ServiceClient<Channel>,
}

impl GrpcNewsClient {
    pub async fn new(mi8_url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Mi8ServiceClient::connect(mi8_url).await?;
        Ok(Self { client })
    }

    pub async fn get_latest_news(
        &mut self,
        limit: i32,
    ) -> Result<Vec<crate::proto::News>, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(GetLatestNewsRequest { limit });
        let response = self.client.get_latest_news(request).await?;
        Ok(response.into_inner().news)
    }

    pub async fn get_latest_news_in_city(
        &mut self,
        city: String,
        limit: i32,
    ) -> Result<Vec<crate::proto::News>, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(GetLatestNewsInCityRequest { city, limit });
        let response = self.client.get_latest_news_in_city(request).await?;
        Ok(response.into_inner().news)
    }
}
