use std::{convert::Infallible, net::SocketAddr};

use hyper::{Request, Body, Response, Method, service::{make_service_fn, service_fn}, Server, StatusCode};
use oles::response::IntoResponse;
use params::ContextHandler;
use serde_json::json;

use crate::{
    response::{Resp}
};

pub mod handler;
pub mod params;
pub mod response;

#[tokio::main]
async fn main() {

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));


    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handler))
    });

    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}


async fn ping(_ctx: ContextHandler, _req: Request<Body>) -> Resp {
    "PONG".into_response()
}

async fn t_ping(_ctx: ContextHandler, _req: Request<Body>) -> Resp {
    json!({"haha": 0}).with_status(StatusCode::OK)
}


async fn handler(_req: Request<Body>) -> Result<Response<Body>, Infallible> {

    let route = vec![
        handler::Handler::new("/ping/", Method::GET, Box::new(t_ping)).await,
        handler::Handler::new("/ping", Method::GET, Box::new(ping)).await
    ];

     let handler = handler::Handlers::new(route).await;

    handler.route(_req).await
}