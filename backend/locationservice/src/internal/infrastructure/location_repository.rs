use std::collections::HashMap;

use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;
use uuid::Uuid;

use crate::internal::domain::location_repository::LocationRepository;
use crate::internal::domain::models::id::UserId;
use crate::internal::domain::models::location::Locations;
use crate::internal::infrastructure::error::DbError;

pub struct LocationRepositoryImpl {
    client: Client,
}

#[async_trait]
impl LocationRepository for LocationRepositoryImpl {
    fn new(client: Client) -> Self {
        Self { client }
    }
    async fn save(&self, locations: Locations) -> Result<(), DbError> {
        let mut item = HashMap::new();
        item.insert("user_id".to_string(), AttributeValue::S(locations.user_id.to_string()));

        // Vec<Location>をDynamoDBのList形式に変換
        let location_list: Vec<AttributeValue> = locations
            .locations
            .iter()
            .map(|loc| {
                let mut location_map = HashMap::new();
                location_map
                    .insert("latitude".to_string(), AttributeValue::N(loc.latitude.to_string()));
                location_map
                    .insert("longitude".to_string(), AttributeValue::N(loc.longitude.to_string()));
                location_map
                    .insert("timestamp".to_string(), AttributeValue::N(loc.timestamp.to_string()));
                AttributeValue::M(location_map)
            })
            .collect();

        item.insert("locations".to_string(), AttributeValue::L(location_list));
        item.insert("is_finished".to_string(), AttributeValue::Bool(locations.is_finished));

        self.client.put_item().table_name("locations").set_item(Some(item)).send().await?;

        Ok(())
    }
    async fn get(&self, user_id: UserId) -> Result<Locations, DbError> {
        let result = self
            .client
            .get_item()
            .table_name("locations")
            .key("user_id", AttributeValue::S(user_id.to_string()))
            .send()
            .await
            .map_err(|e| DbError::Infrastructure(e.to_string()))?;

        if let Some(item) = result.item {
            // DynamoDBのList形式からVec<Location>に変換
            let locations_vec = if let Some(AttributeValue::L(location_list)) =
                item.get("locations")
            {
                location_list
                    .iter()
                    .filter_map(|attr| {
                        if let AttributeValue::M(location_map) = attr {
                            let latitude =
                                location_map.get("latitude")?.as_n().ok()?.parse::<f64>().ok()?;
                            let longitude =
                                location_map.get("longitude")?.as_n().ok()?.parse::<f64>().ok()?;
                            let timestamp =
                                location_map.get("timestamp")?.as_n().ok()?.parse::<i64>().ok()?;

                            Some(crate::internal::domain::models::location::Location {
                                latitude,
                                longitude,
                                timestamp,
                            })
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                Vec::new()
            };

            let user_id_str =
                item.get("user_id").and_then(|v| v.as_s().ok()).map(|s| s.as_str()).unwrap_or("");
            let user_id = Uuid::parse_str(user_id_str)
                .map(UserId::from_uuid)
                .map_err(|_| DbError::Infrastructure("Invalid UUID format".to_string()))?;

            let locations = Locations {
                user_id,
                locations: locations_vec,
                is_finished: item
                    .get("is_finished")
                    .and_then(|v| v.as_bool().ok())
                    .copied()
                    .unwrap_or(false),
            };
            Ok(locations)
        } else {
            Err(DbError::NotFound("Locations not found".to_string()))
        }
    }
}
