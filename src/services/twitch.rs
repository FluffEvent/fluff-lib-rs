use std::env;

use crate::errors::FluffError;
use crate::models::twitch::{OAuthResponse, User};


fn get_client_id() -> Result<String, FluffError> {
    match env::var("TWITCH_CLIENT_ID") {
        Ok(client_id) => Ok(client_id),
        Err(_) => Err(FluffError::new_u16(
            500,
            "TwitchClientIdMissing",
            "Missing TWITCH_CLIENT_ID in environment variables",
            true,
        )),
    }
}

fn get_client_secret() -> Result<String, FluffError> {
    match env::var("TWITCH_CLIENT_SECRET") {
        Ok(client_secret) => Ok(client_secret),
        Err(_) => Err(FluffError::new_u16(
            500,
            "TwitchClientSecretMissing",
            "Missing TWITCH_CLIENT_SECRET in environment variables",
            true,
        )),
    }
}

fn get_redirect_uri() -> String {
    env::var("TWITCH_REDIRECT_URI").unwrap_or_else(|_| "https://fluffevent.fr".to_string())
}

pub async fn get_oauth_from_code(code: String) -> Result<OAuthResponse, FluffError> {
    let url = get_redirect_uri();
    let url = urlencoding::encode(&url);

    let client = reqwest::Client::new();
    let response = client
        .post("https://id.twitch.tv/oauth2/token")
        .form(&[
            ("client_id", get_client_id()?),
            ("client_secret", get_client_secret()?),
            ("code", code),
            ("grant_type", String::from("authorization_code")),
            ("redirect_uri", url.into_owned()),
        ])
        .send()
        .await
        .map_err(|err| {
            FluffError::new_u16(
                500,
                "TwitchOAuthError",
                "Unable to get OAuth token from Twitch",
                true,
            )
            .add_context(&err.to_string())
        })?;

    response.json::<OAuthResponse>()
        .await
        .map_err(|err| {
            FluffError::new_u16(
                500,
                "TwitchOAuthError",
                "Unable to parse OAuth token from Twitch",
                true,
            )
            .add_context(&err.to_string())
        })
}

pub async fn get_oauth_for_app() -> Result<OAuthResponse, FluffError> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://id.twitch.tv/oauth2/token")
        .form(&[
            ("client_id", get_client_id()?),
            ("client_secret", get_client_secret()?),
            ("grant_type", String::from("client_credentials")),
        ])
        .send()
        .await
        .map_err(|err| {
            FluffError::new_u16(
                500,
                "TwitchOAuthError",
                "Unable to get OAuth token from Twitch",
                true,
            )
            .add_context(&err.to_string())
        })?;

    response.json::<OAuthResponse>()
        .await
        .map_err(|err| {
            FluffError::new_u16(
                500,
                "TwitchOAuthError",
                "Unable to parse OAuth token from Twitch",
                true,
            )
            .add_context(&err.to_string())
        })
}

pub async fn get_users(oauth: &str, users_id: Vec<String>, users_login: Vec<String>) -> Result<Vec<User>, FluffError> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.twitch.tv/helix/users")
        .header("Authorization", format!("Bearer {}", oauth))
        .header("Client-Id", get_client_id()?)
        .query(&[
            ("id", users_id.join(",")),
            ("login", users_login.join(",")),
        ])
        .send()
        .await
        .map_err(|err| {
            FluffError::new_u16(
                500,
                "TwitchUserError",
                "Unable to get user from Twitch",
                true,
            )
            .add_context(&err.to_string())
        })?;

    response.json::<Vec<User>>().await.map_err(|err| {
        FluffError::new_u16(
            500,
            "TwitchUserError",
            "Unable to parse user from Twitch",
            true,
        )
        .add_context(&err.to_string())
    })
}
