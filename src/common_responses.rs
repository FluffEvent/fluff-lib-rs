use crate::errors::FluffError;
use lambda_http::{Body, Response};

fn http_response(code: u16, mimetype: &str, headers: Vec<(&str, &str)>, content: &str) -> Result<Response<Body>, FluffError> {
    let mut response = Response::builder()
        .status(code)
        .header("content-type", &format!("{mimetype}; charset=utf-8"));
    for (key, value) in headers {
        response = response.header(key, value);
    }
    response
        .body(content.into())
        .map_err(|err| {
            FluffError::new_u16(
                500,
                "LambdaResponseError",
                "Failed to generate HTTP Response",
                true,
            )
            .add_context(&err.to_string())
        })
}

pub fn ok_200_json(content: &str) -> Result<Response<Body>, FluffError> {
    http_response(200, "application/json", vec![], content)
}

pub fn ok_200_html(content: &str) -> Result<Response<Body>, FluffError> {
    http_response(200, "text/html", vec![], content)
}

pub fn redirect_302(link: &str) -> Result<Response<Body>, FluffError> {
    http_response(
        302,
        "text/html",
        vec![("Location", link)],
        &format!(
            "<html><head><title>Fluff Event</title></head><body><a href=\"{}\">If you are not redirected automaticaly, click here</a></body></html>",
            link
        )
    )
}

pub fn successful_auth_302(link: &str, cookie: &str) -> Result<Response<Body>, FluffError> {
    http_response(
        302,
        "text/html",
        vec![("Set-Cookie", cookie), ("Location", link)],
        &format!(
            "<html><head><title>Fluff Event</title></head><body>You are successfully authentificated. <a href=\"{}\">If you are not redirected automaticaly, click here</a></body></html>",
            link
        )
    )
}
