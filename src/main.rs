use std::{convert::Infallible, net::SocketAddr};

use hyper::{Request, Body, Response, service::{make_service_fn, service_fn}, Server};


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


async fn handler(_req: Request<Body>) -> Result<Response<Body>, Infallible> {

    let route = vec![];

     let handler = handler::Handlers::new(route).await;

    handler.route(_req).await
}