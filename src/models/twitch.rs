use serde::{Serialize, Deserialize};

use crate::errors::FluffError;
use crate::services::twitch;


#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub refresh_token: Option<String>,
    pub scope: Vec<String>,
    pub token_type: String,
}


impl OAuthResponse {
    pub async fn from_code(code: String) -> Result<OAuthResponse, FluffError> {
        twitch::get_oauth_from_code(code).await
    }

    pub async fn for_app() -> Result<OAuthResponse, FluffError> {
        twitch::get_oauth_for_app().await
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub r#type: String,
    pub broadcaster_type: String,
    pub description: String,
    pub profile_image_url: String,
    pub offline_image_url: String,
    pub view_count: Option<u64>,
    pub email: Option<String>,
    pub created_at: String,
}


