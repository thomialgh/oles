use hyper::{Body, Response, StatusCode, http::HeaderValue};
use serde_json::{self, json};

pub async fn response_builder(body: serde_json::Value) -> Result<Response<Body>, hyper::http::Error> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("Content-Type", "application/json")
        .body(Body::from(body.to_string()))
}

pub async fn response_not_found() -> Result<Response<Body>, hyper::http::Error> {
    
    let body = json!({
        "status": StatusCode::NOT_FOUND.as_u16(),
        "msg": "NOT FOUND" 
    });

    response_builder(body).await
}

pub async fn response_method_not_allowed() -> Result<Response<Body>, hyper::http::Error> {
    let body = json!(
        {
            "status": StatusCode::METHOD_NOT_ALLOWED.as_u16(),
            "msg": "Method Not Allowed"
        }
    );

    response_builder(body).await
}

pub async fn response_internal_server_err() -> Response<Body> {

    let body = json!({
        "status": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        "msg": "Internal Server Error",
    });
    let mut resp = Response::new(Body::from(body.to_string()));
    *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    if let Ok(hv) = HeaderValue::from_str("application/json") {
        resp.headers_mut().append("Content-Type", hv);
    }
    resp
}