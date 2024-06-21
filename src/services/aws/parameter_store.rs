use crate::errors::FluffError;
use aws_config::BehaviorVersion;

struct AwsParametersPage {
    parameters: Vec<(String, String)>,
    next_token: Option<String>,
}

pub async fn get_parameter(parameter_name: &str) -> Result<String, FluffError> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_ssm::Client::new(&config);

    let aws_output = client
        .get_parameter()
        .name(parameter_name)
        .send()
        .await
        .map_err(|err| {
            FluffError::new_u16(
                500,
                "ParameterStoreError",
                "Unable to read Parameter Store",
                true,
            )
            .add_context(&err.to_string())
        })?;

    if let Some(parameter) = aws_output.parameter {
        if let Some(value) = parameter.value {
            return Ok(value);
        }
    }
    Err(FluffError::new_u16(
        404,
        "ParameterNotFound",
        "The requested parameter was not found",
        true,
    )
    .add_context(parameter_name))
}

async fn int_get_parameters_path(
    client: &aws_sdk_ssm::Client,
    token: &str,
    parameter_path: &str,
) -> Result<AwsParametersPage, FluffError> {
    let mut response = AwsParametersPage {
        parameters: Vec::new(),
        next_token: None,
    };

    let aws_output = client
        .get_parameters_by_path()
        .path(parameter_path)
        .next_token(token)
        .send()
        .await
        .map_err(|err| {
            FluffError::new_u16(
                500,
                "ParameterStoreError",
                "Unable to read Parameter Store",
                true,
            )
            .add_context(&err.to_string())
        })?;

    if let Some(next_token) = aws_output.next_token {
        response.next_token = Some(next_token);
    }
    if let Some(parameters) = aws_output.parameters {
        for parameter in parameters {
            if let Some(value) = parameter.value {
                if let Some(name) = parameter.name {
                    response.parameters.push((name, value));
                }
            }
        }
    }

    Ok(response)
}

pub async fn get_parameters(parameter_path: &str) -> Result<Vec<(String, String)>, FluffError> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_ssm::Client::new(&config);

    let mut parameters = Vec::new();
    let mut next_token = String::from("");

    loop {
        let response =
            int_get_parameters_path(&client, next_token.as_ref(), parameter_path).await?;
        parameters.extend(response.parameters);
        if let Some(token) = response.next_token {
            next_token = token;
        } else {
            break;
        }
    }

    Ok(parameters)
}

pub async fn put_parameters(
    parameter_name: &str,
    value: &str,
    ptype: aws_sdk_ssm::types::ParameterType,
) -> Result<(), FluffError> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_ssm::Client::new(&config);

    client
        .put_parameter()
        .name(parameter_name)
        .value(value)
        .r#type(ptype)
        .overwrite(true)
        .tier(aws_sdk_ssm::types::ParameterTier::Standard)
        .data_type("text")
        .send()
        .await
        .map_err(|err| {
            FluffError::new_u16(
                500,
                "ParameterStoreError",
                "Unable to write in Parameter Store",
                true,
            )
            .add_context(parameter_name)
            .add_context(&err.to_string())
        })?;

    Ok(())
}

pub async fn delete_parameters(parameter_name: &str) -> Result<(), FluffError> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_ssm::Client::new(&config);

    client
        .delete_parameter()
        .name(parameter_name)
        .send()
        .await
        .map_err(|err| {
            FluffError::new_u16(
                500,
                "ParameterStoreError",
                "Unable to delete in Parameter Store",
                true,
            )
            .add_context(parameter_name)
            .add_context(&err.to_string())
        })?;

    Ok(())
}
