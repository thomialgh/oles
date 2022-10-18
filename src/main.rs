use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use hyper::{service::{make_service_fn, service_fn}, Server};

use crate::router::route;


pub mod params;
pub mod response;
pub mod router;

#[tokio::main]
async fn main() {

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let router_shared = Arc::new(router());
    let shared_service = Arc::new(Svc);
    let make_service = make_service_fn(move |_conn| 
    {
        let router_shared = Arc::clone(&router_shared);
        let shared_service = Arc::clone(&shared_service);
        async move {
            Ok::<_, Infallible>(service_fn(move |_req| {
                let router_shared = Arc::clone(&router_shared);
                let shared_service = Arc::clone(&shared_service);
                route(_req, router_shared, shared_service)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}


struct Svc;


fn router() -> router::Router<Svc> 
{
    let router = router::Router::new();

    router
}

