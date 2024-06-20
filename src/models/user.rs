use std::collections::HashMap;
use aws_sdk_dynamodb::types::AttributeValue;

use crate::errors::FluffError;
use crate::models::user_jwt::UserJWT;
use crate::services::aws::dynamodb::DynamoItem;
use crate::services::aws::TABLE_USERS;

pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub email: Option<String>,
    pub profile_picture: Option<String>,
    pub permissions: Vec<String>,
}

impl User {
    pub fn from_jwt(jwt: &UserJWT) -> User {
        User {
            id: jwt.sub.clone(),
            username: jwt.name.clone(),
            display_name: jwt.display_name.clone(),
            email: None,
            profile_picture: Some(jwt.picture.clone()),
            permissions: jwt.scope.clone(),
        }
    }

    fn from_dynamo_item(item: DynamoItem) -> Result<User, FluffError> {
        let id = item.get_string("id")?;
        let username = item.get_string("username")?;
        let display_name = item.get_string("display_name")?;
        let email = item.get_string_opt("email");
        let profile_picture = item.get_string_opt("profile_picture");
        let permissions = item.get_strings_vec("permissions")?;

        Ok(User {
            id,
            username,
            display_name,
            email,
            profile_picture,
            permissions,
        })
    }

    fn into_dynamo_hashmap(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("username".to_string(), AttributeValue::S(self.username.clone()));
        item.insert("display_name".to_string(), AttributeValue::S(self.display_name.clone()));
        if let Some(email) = &self.email {
            item.insert("email".to_string(), AttributeValue::S(email.clone()));
        }
        if let Some(profile_picture) = &self.profile_picture {
            item.insert("profile_picture".to_string(), AttributeValue::S(profile_picture.clone()));
        }
        item.insert("permissions".to_string(), AttributeValue::Ss(self.permissions.clone()));
        item
    }

    pub async fn from_db(id: String) -> Result<User, FluffError> {
        let mut query = HashMap::new();
        query.insert("id".to_string(), AttributeValue::S(id));

        let item = crate::services::aws::dynamodb::get_item(TABLE_USERS, query, false).await?;

        User::from_dynamo_item(item)
    }

    pub async fn to_db(&self) -> Result<bool, FluffError> {
        let item = self.into_dynamo_hashmap();

        crate::services::aws::dynamodb::insert_item(TABLE_USERS, item).await
    }
}
