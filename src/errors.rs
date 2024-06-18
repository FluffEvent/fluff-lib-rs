use http::status::StatusCode;
use lambda_http::{Body, Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FluffError {
    pub http_code: u16,
    pub http_reason: String,
    pub error_name: String,
    pub error_description: String,
    pub can_retry: bool,
    pub context: Vec<String>,
}

impl FluffError {
    #[allow(unused)]
    pub fn new(http: StatusCode, name: &str, description: &str, can_retry: bool) -> FluffError {
        FluffError {
            http_code: http.as_u16(),
            http_reason: String::from(http.canonical_reason().unwrap_or("Error")),
            error_name: String::from(name),
            error_description: String::from(description),
            can_retry,
            context: vec![],
        }
    }
    #[allow(unused)]
    pub fn new_u16(http: u16, name: &str, description: &str, can_retry: bool) -> FluffError {
        let temp = match StatusCode::from_u16(http) {
            Ok(stcode) => stcode.canonical_reason().unwrap_or("Error"),
            Err(_) => "Error",
        };
        FluffError {
            http_code: http,
            http_reason: String::from(temp),
            error_name: String::from(name),
            error_description: String::from(description),
            can_retry,
            context: vec![],
        }
    }
    #[allow(unused)]
    pub fn add_context(mut self, text: &str) -> Self {
        self.context.push(String::from(text));
        self
    }
    #[allow(unused)]
    pub fn reset_context(mut self) -> Self {
        self.context = vec![];
        self
    }

    #[allow(unused)]
    pub fn to_http_response(&self) -> Result<Response<Body>, Error> {
        Ok(Response::builder()
            .status(self.http_code)
            .header("content-type", "application/json; charset=utf-8")
            .body(json!(self).to_string().into())
            .map_err(Box::new)?)
    }
}
