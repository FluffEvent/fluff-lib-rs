use std::env;
use std::fs::read;

use crate::errors::FluffError;
use crate::services::aws::s3;
use crate::services::aws::BUCKET_PROD;

fn read_private_from_env() -> Result<Vec<u8>, FluffError> {
    match env::var("PRIVATE_KEY_CONTENT") {
        Ok(pem_content) => Ok(pem_content.into_bytes()),
        Err(_) => Err(FluffError::new_u16(
            500,
            "PrivateKeyMissing",
            "Missing PRIVATE_KEY_CONTENT in environment variables",
            true,
        ))
    }
}

fn read_private_from_file() -> Result<Vec<u8>, FluffError> {
    match env::var("PRIVATE_KEY_FILE") {
        Ok(pem_file) => read(pem_file).map_err(|err| {
            FluffError::new_u16(
                500,
                "PrivateKeyUnavailable",
                "Unable to read private key",
                true,
            )
            .add_context(&err.to_string())
        }),
        Err(_) => Err(FluffError::new_u16(
            500,
            "PrivateKeyMissing",
            "Missing PRIVATE_KEY_FILE in environment variables",
            true,
        ))
    }
}

async fn read_private_from_s3() -> Result<Vec<u8>, FluffError> {
    match env::var("PRIVATE_KEY_S3_PATH") {
        Ok(pem_file) => s3::read_object(BUCKET_PROD, &pem_file).await,
        Err(_) => Err(FluffError::new_u16(
            500,
            "PrivateKeyMissing",
            "Missing PRIVATE_KEY_S3_PATH in environment variables",
            true,
        ))
    }
}


pub async fn read_private_key() -> Result<Vec<u8>, FluffError> {
    read_private_from_s3().await
        .or_else(|_| read_private_from_env())
        .or_else(|_| read_private_from_file())
}




fn read_public_from_env() -> Result<Vec<u8>, FluffError> {
    match env::var("PUBLIC_KEY_CONTENT") {
        Ok(pub_content) => Ok(pub_content.into_bytes()),
        Err(_) => Err(FluffError::new_u16(
            500,
            "PublicKeyMissing",
            "Missing PUBLIC_KEY_CONTENT in environment variables",
            true,
        ))
    }
}

fn read_public_from_file() -> Result<Vec<u8>, FluffError> {
    match env::var("PUBLIC_KEY_FILE") {
        Ok(pub_file) => read(pub_file).map_err(|err| {
            FluffError::new_u16(
                500,
                "PublicKeyUnavailable",
                "Unable to read public key",
                true,
            )
            .add_context(&err.to_string())
        }),
        Err(_) => Err(FluffError::new_u16(
            500,
            "PublicKeyMissing",
            "Missing PUBLIC_KEY_FILE in environment variables",
            true,
        ))
    }
}

async fn read_public_from_s3() -> Result<Vec<u8>, FluffError> {
    match env::var("PUBLIC_KEY_S3_PATH") {
        Ok(pub_file) => s3::read_object(BUCKET_PROD, &pub_file).await,
        Err(_) => Err(FluffError::new_u16(
            500,
            "PublicKeyMissing",
            "Missing PUBLIC_KEY_S3_PATH in environment variables",
            true,
        ))
    }
}

pub async fn read_public_key() -> Result<Vec<u8>, FluffError> {
    read_public_from_s3().await
        .or_else(|_| read_public_from_env())
        .or_else(|_| read_public_from_file())
}
