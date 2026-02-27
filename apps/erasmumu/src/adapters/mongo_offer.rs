use async_trait::async_trait;
use mongodb::bson::{Bson, doc, to_bson};
use mongodb::{Client, Collection};
use uuid::Uuid;

use zukmove_core::domain::entities::offer::Offer;
use zukmove_core::domain::ports::{DomainError, OfferRepository};

pub struct MongoOfferRepository {
    collection: Collection<Offer>,
}

impl MongoOfferRepository {
    pub fn new(client: &Client, database: &str) -> Self {
        let db = client.database(database);
        let collection = db.collection::<Offer>("offers");
        Self { collection }
    }
}

#[async_trait]
impl OfferRepository for MongoOfferRepository {
    async fn save(&self, offer: &Offer) -> Result<Offer, DomainError> {
        self.collection
            .insert_one(offer)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
        Ok(offer.clone())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Offer, DomainError> {
        let id_bson = uuid_to_bson(id)?;
        let filter = doc! { "id": id_bson };
        let offer = self
            .collection
            .find_one(filter)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound(format!("Offer with id {} not found", id)))?;

        // Business rule: do not return unavailable offers
        if !offer.available {
            return Err(DomainError::NotFound(format!(
                "Offer with id {} is not available",
                id
            )));
        }

        Ok(offer)
    }

    async fn find_by_domain(&self, domain: &str) -> Result<Vec<Offer>, DomainError> {
        let filter = doc! { "domain": domain, "available": true };
        let mut cursor = self
            .collection
            .find(filter)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        let mut offers = Vec::new();
        while cursor
            .advance()
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
        {
            let offer = cursor
                .deserialize_current()
                .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
            offers.push(offer);
        }
        Ok(offers)
    }

    async fn find_by_city(&self, city: &str) -> Result<Vec<Offer>, DomainError> {
        let filter = doc! { "city": city, "available": true };
        let mut cursor = self
            .collection
            .find(filter)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        let mut offers = Vec::new();
        while cursor
            .advance()
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
        {
            let offer = cursor
                .deserialize_current()
                .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
            offers.push(offer);
        }
        Ok(offers)
    }

    async fn update(&self, offer: &Offer) -> Result<Offer, DomainError> {
        let id_bson = uuid_to_bson(offer.id)?;
        let filter = doc! { "id": id_bson };

        let result = self
            .collection
            .replace_one(filter, offer)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        if result.matched_count == 0 {
            return Err(DomainError::NotFound(format!(
                "Offer with id {} not found",
                offer.id
            )));
        }

        Ok(offer.clone())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        let id_bson = uuid_to_bson(id)?;
        let filter = doc! { "id": id_bson };
        let result = self
            .collection
            .delete_one(filter)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        if result.deleted_count == 0 {
            return Err(DomainError::NotFound(format!(
                "Offer with id {} not found",
                id
            )));
        }
        Ok(())
    }
}

fn uuid_to_bson(id: Uuid) -> Result<Bson, DomainError> {
    to_bson(&id).map_err(|e| DomainError::InfrastructureError(e.to_string()))
}
