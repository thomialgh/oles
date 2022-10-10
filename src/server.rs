use std::{convert::Infallible, net::SocketAddr, error::Error};
use hyper::{Response, Request, Body, server::conn::Http};
use tokio::net::TcpListener;

use crate::{handler, router, response::{response_not_found, response_internal_server_err}};




fn _handle(c: handler::ContextHandler, _req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("{:?}", c);
    Ok(response_not_found().unwrap_or(response_internal_server_err()))
}

pub fn make_service() -> Result<router::RouterService, Box<dyn Error>> {
    let mut router = router::Router::new();
    router.get("/ping/(?P<id>[0-9]+)", _handle)?;

    Ok(router::RouterService::new(router))
}


pub struct Server {
    address: SocketAddr,
}

impl Server {
    pub async fn new(address: SocketAddr) -> Self {
        Self { address}
    }

    pub async fn run(&self, service: fn() -> Result<router::RouterService, Box<dyn Error>>) {
        let addr = SocketAddr::from(self.address);


        let listener = TcpListener::bind(addr).await.unwrap();

        loop {
            let service = service().unwrap();
            let (stream, _ ) = listener.accept().await.unwrap();
            tokio::task::spawn( async move {
                if let Err(err) = Http::new()
                    .serve_connection(stream, service)
                    .await {
                        eprintln!("Failed to init service {}", err);
                    }

            });
        }
    }
}

