use std::collections::HashMap;

use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;

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
        let timestamp =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()
                as i64;

        // sort_keyとしてtimestampを使用して複数レコードを保存
        item.insert("user_id".to_string(), AttributeValue::S(locations.user_id.to_string()));
        item.insert(
            "location_id".to_string(),
            AttributeValue::S(locations.location_id.to_string()),
        );
        item.insert("timestamp".to_string(), AttributeValue::N(timestamp.to_string()));

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
    async fn get(&self, user_id: UserId) -> Result<Vec<Locations>, DbError> {
        // user_idでスキャンし、is_finished=falseのもののみを取得
        let result = self
            .client
            .scan()
            .table_name("locations")
            .filter_expression("user_id = :user_id AND is_finished = :is_finished")
            .expression_attribute_values(":user_id", AttributeValue::S(user_id.to_string()))
            .expression_attribute_values(":is_finished", AttributeValue::Bool(false))
            .send()
            .await
            .map_err(|e| DbError::Infrastructure(e.to_string()))?;

        let mut locations_list = Vec::new();

        if let Some(items) = result.items {
            for item in items {
                let location_id = item
                    .get("location_id")
                    .and_then(|v| v.as_s().ok())
                    .and_then(|s| {
                        crate::internal::domain::models::id::LocationId::from_string(s.clone()).ok()
                    })
                    .ok_or_else(|| {
                        DbError::Infrastructure(
                            "Invalid location_id format in saved data".to_string(),
                        )
                    })?;

                if let Some(AttributeValue::L(location_list)) = item.get("locations") {
                    let locations_vec: Vec<crate::internal::domain::models::location::Location> =
                        location_list
                            .iter()
                            .filter_map(|attr| {
                                if let AttributeValue::M(location_map) = attr {
                                    let latitude = location_map
                                        .get("latitude")?
                                        .as_n()
                                        .ok()?
                                        .parse::<f64>()
                                        .ok()?;
                                    let longitude = location_map
                                        .get("longitude")?
                                        .as_n()
                                        .ok()?
                                        .parse::<f64>()
                                        .ok()?;
                                    let timestamp = location_map
                                        .get("timestamp")?
                                        .as_n()
                                        .ok()?
                                        .parse::<i64>()
                                        .ok()?;

                                    Some(crate::internal::domain::models::location::Location {
                                        latitude,
                                        longitude,
                                        timestamp,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();

                    locations_list.push(Locations {
                        location_id,
                        user_id,
                        locations: locations_vec,
                        is_finished: false,
                    });
                }
            }
        }

        Ok(locations_list)
    }
}
