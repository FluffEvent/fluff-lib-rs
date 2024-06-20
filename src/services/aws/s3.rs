use aws_config::BehaviorVersion;

use crate::errors::FluffError;

pub async fn read_object(bucket: &str, object_key: &str) -> Result<Vec<u8>, crate::errors::FluffError> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_s3::Client::new(&config);

    let response = client
        .get_object()
        .bucket(bucket)
        .key(object_key)
        .send()
        .await
        .map_err(|err| {
            FluffError::new_u16(500, "S3accessdenied", "S3 file cannot be read", true)
                .add_context(bucket)
                .add_context(object_key)
                .add_context(&err.to_string())
        })?;

    let data = response.body.collect().await.map_err(|err| {
        FluffError::new_u16(500, "S3accessdenied", "S3 file cannot be read", true)
            .add_context(bucket)
            .add_context(object_key)
            .add_context(&err.to_string())
    })?;
    Ok(data.to_vec())
}
