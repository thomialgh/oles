use hyper::{Body, Response, StatusCode};
use serde_json::{self, json, Value};

pub async fn response_not_found() -> Resp {
    json!({
        "status": StatusCode::NOT_FOUND.as_u16(),
        "msg": "NOT FOUND" 
    }).with_status(StatusCode::NOT_FOUND)
}

pub async fn response_method_not_allowed() -> Resp {
    json!(
        {
            "status": StatusCode::METHOD_NOT_ALLOWED.as_u16(),
            "msg": "Method Not Allowed"
        }
    ).with_status(StatusCode::METHOD_NOT_ALLOWED)
}

pub async fn response_internal_server_err() -> Resp {
    json!({
        "status": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        "msg": "Internal Server Error",
    }).with_status(StatusCode::INTERNAL_SERVER_ERROR)
}

pub type Resp = Response<Body>;

pub trait IntoResponse: Send + Sized + 'static {
    fn into_response(self) -> Resp;

    fn with_status(self, status: StatusCode) -> Resp {
        let mut r = self.into_response();
        *r.status_mut() = status;
        r
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> Resp {
        Response::new(self.into())
    }
}

impl IntoResponse for Value {
    fn into_response(self) -> Resp {
        Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(self.to_string()))
            .unwrap_or("Internal Server Error".with_status(StatusCode::INTERNAL_SERVER_ERROR))
    }
}