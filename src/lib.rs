use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use hyper::service::{make_service_fn, service_fn};
use router::Router;

pub mod response;
pub mod params;

pub mod router;

pub struct Server<S>
where
    S: Send + Sync + 'static
{
    addr: SocketAddr,
    svc: Arc<S>,
    router: Arc<Router<S>>
}

impl<S> Server<S>
where
    S: Send + Sync + 'static
{
    pub fn new(addr: SocketAddr, svc: Arc<S>, router: Arc<Router<S>>) -> Self {
        Self {addr, svc, router}
    }

    pub async fn run(&self) -> Result<(), hyper::Error> {
        let make_service = make_service_fn(move |_addr| {
            let shared_router = self.router.clone();
            let shared_svc = self.svc.clone();
            async {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let shared_router = shared_router.clone();
                    let shared_svc = shared_svc.clone();
                    router::route(req, shared_router, shared_svc)
                }))
            }
        });

        hyper::server::Server::bind(&self.addr).serve(make_service).await   
    }
}
