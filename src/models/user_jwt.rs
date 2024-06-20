use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use jsonwebtoken;

use crate::errors::FluffError;
use crate::models::user::User;


#[derive(Debug, Serialize, Deserialize)]
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

    pub async fn sign(&self) -> Result<String, FluffError> {
        let private_key = crate::services::rsa_keys::read_private_key().await?;
        
        let key = jsonwebtoken::EncodingKey::from_rsa_pem(&private_key).map_err(|err| {
            FluffError::new_u16(500, "PrivateKeyError", "Private key is malformatted", true)
                .add_context(&err.to_string())
        })?;
    
        let mut head = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS512);
        head.typ = Some(String::from("JWT"));
    
        jsonwebtoken::encode(&head, &self, &key).map_err(|err| {
            FluffError::new_u16(500, "JWTEncodingFailed", "Unable to encode JWT", true)
                .add_context(&err.to_string())
        })
    }

    pub async fn verify(jwt: &str) -> Result<UserJWT, FluffError> {
        let public_key = crate::services::rsa_keys::read_public_key().await?;
        
        let key = jsonwebtoken::DecodingKey::from_rsa_pem(&public_key).map_err(|err| {
            FluffError::new_u16(500, "PublicKeyError", "Public key is malformatted", true)
                .add_context(&err.to_string())
        })?;
    
        let mut validating = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS512);
        validating.set_issuer(&["https://auth.fluffevent.fr"]);
        validating.set_audience(&["fluffevent.fr"]);
        validating.leeway = 300;
        validating.validate_exp = true;
        validating.validate_nbf = true;
        validating.set_required_spec_claims(&[
            "iss",
            "sub",
            "aud",
            "nbf",
            "exp",
            "iat",
            "name",
            "display_username",
            "picture",
            "scope",
        ]);
    
        let decoded: jsonwebtoken::TokenData<UserJWT> =
            jsonwebtoken::decode(jwt, &key, &validating).map_err(|err| {
                FluffError::new_u16(
                    500,
                    "JWTDecodingFailed",
                    "Unable to decode and validate JWT",
                    true,
                )
                .add_context(&err.to_string())
            })?;
        Ok(decoded.claims)
    }
}
