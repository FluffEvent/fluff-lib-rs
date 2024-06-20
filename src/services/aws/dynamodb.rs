use std::collections::HashMap;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;

use crate::errors::FluffError;

pub struct DynamoItem {
    data: HashMap<String, AttributeValue>
}

impl DynamoItem {
    pub fn get_string(&self, key: &str) -> Result<String, FluffError> {
        self.data.get(key)
            .ok_or(FluffError::new_u16(500, "DatabaseError", "Key not found in database item", true))
            .and_then(|v| v.as_s()
                .or(Err(FluffError::new_u16(500, "DatabaseError", "Value is not a string", true)))
            ).map(|v| v.clone())
    }
    
    pub fn get_string_opt(&self, key: &str) -> Option<String> {
        self.data.get(key)
            .and_then(|v| v.as_s().ok())
            .map(|v| v.clone())
    }

    pub fn get_strings_vec(&self, key: &str) -> Result<Vec<String>, FluffError> {
        self.data.get(key)
            .ok_or(FluffError::new_u16(500, "DatabaseError", "Key not found in database item", true))
            .and_then(|v| v.as_ss()
                .or(Err(FluffError::new_u16(500, "DatabaseError", "Value is not a string set", true)))
            ).map(|v| v.clone())
    }
}

pub async fn insert_item(table: &str, item: HashMap<String, AttributeValue>) -> Result<bool, FluffError> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_dynamodb::Client::new(&config);

    client.put_item()
        .table_name(table)
        .set_item(Some(item))
        .send()
        .await
        .map_err(|err| {
            FluffError::new_u16(
                500,
                "DatabaseError",
                "Failed to insert item in database",
                true,
            )
            .add_context(table)
            .add_context(&err.to_string())
        })?;

    Ok(true)
}

pub async fn get_item(table: &str, keys: HashMap<String, AttributeValue>, consistent: bool) -> Result<DynamoItem, FluffError> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_dynamodb::Client::new(&config);

    let output = client
        .get_item()
        .table_name(table)
        .set_key(Some(keys))
        .consistent_read(consistent)
        .send()
        .await
        .map_err(|err| {
            FluffError::new_u16(
                500,
                "DatabaseError",
                "Failed to get item in database",
                true,
            )
            .add_context(table)
            .add_context(&err.to_string())
        })?;

    output.item.ok_or(FluffError::new_u16(
        500,
        "DatabaseError",
        "Item not found in database or expired",
        true,
    ))
    .map(|item| DynamoItem { data: item })
}
