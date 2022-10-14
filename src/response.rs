use hyper::{Body, Response, StatusCode};
use serde::Serialize;
use serde_json::{self, json, Value};

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

impl IntoResponse for String {
    fn into_response(self) -> Resp {
        Response::new(Body::from(self))
    }
}

impl<T: Serialize + Send + Sized + 'static> IntoResponse for T 
where T: ResponseJson
{
    fn into_response(self) -> Resp {
        let val = match serde_json::to_value(&self) {
            Ok(v) => v,
            _ => {return "Internal Server Error".with_status(StatusCode::INTERNAL_SERVER_ERROR)}
        };

        val.into_response()
    }
}

pub trait ResponseJson: Serialize + Send + Sized + 'static {
}

#[derive(Serialize)]
pub struct ResponseData<T> 
where
    T: Serialize + Send + Sized + 'static
{
    status: u16,
    msg: Option<String>,
    data: T
}

impl<T> ResponseData<T>
where
    T: Serialize + Send + Sized + 'static
{
    pub fn new(status_code: StatusCode, msg: Option<String>, data: T) -> Self {
        let status = status_code.as_u16();
        Self { status, msg, data }
    }
}

impl<T> ResponseJson for ResponseData<T>
where
    T: Serialize + Send + Sized + 'static{}

#[derive(Serialize)]
pub struct ResponseMsg {
    status: u16,
    msg: String,
}

impl ResponseJson for ResponseMsg {}

impl ResponseMsg {
    pub fn new(status_code: StatusCode, msg: String) -> Self {
        Self { status: status_code.as_u16(), msg }
    }
}


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

    fn ok(self) -> Resp {
        self.with_status(StatusCode::OK)
    }

    fn bad_request(self) -> Resp {
        self.with_status(StatusCode::BAD_REQUEST)
    }

    fn method_not_allowed(self) -> Resp {
        self.with_status(StatusCode::METHOD_NOT_ALLOWED)
    }

    fn unauthorized(self) -> Resp {
        self.with_status(StatusCode::UNAUTHORIZED)
    }

    fn not_found(self) -> Resp {
        self.with_status(StatusCode::NOT_FOUND)
    }


}

