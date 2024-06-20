use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::FluffError;
use crate::models::user::User;


pub struct UserJWT {
    pub iss: String,
    pub sub: String,
    pub aud: Vec<String>,
    pub nbf: u64,
    pub exp: u64,
    pub iat: u64,
    pub name: String,
    pub display_name: String,
    pub picture: String,
    pub scope: Vec<String>,
}


impl UserJWT {
    pub fn generate_for_user(user: &User) -> Result<UserJWT, FluffError> {
        let now_as_sec = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|err| {
                FluffError::new_u16(
                    500,
                    "SystemTimeError",
                    "Current system time is before UNIX EPOCH",
                    true,
                )
                .add_context(&err.to_string())
            })?
            .as_secs();
        let validity: u64 = 60 * 60 * 24 * 7;

        Ok(UserJWT {
            iss: String::from("https://auth.fluffevent.fr"),
            sub: user.id.clone(),
            aud: vec![String::from("fluffevent.fr")],
            nbf: now_as_sec - 300,
            exp: now_as_sec + validity,
            iat: now_as_sec,
            name: user.username.clone(),
            display_name: user.display_name.clone(),
            picture: user.profile_picture.clone().unwrap_or("".to_string()),
            scope: user.permissions.clone(),
        })
    }
}
