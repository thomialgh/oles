use std::{ net::SocketAddr, sync::Arc};

use oles::Server;





pub mod params;
pub mod response;
pub mod router;

#[tokio::main]
async fn main() {

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let router_shared = Arc::new(router());
    let shared_service = Arc::new(Svc);

    let server = Server::new(addr, shared_service.clone(), router_shared.clone());

    if let Err(err) = server.run().await {
        eprintln!("{}", err)
    }

}


struct Svc;


fn router() -> oles::router::Router<Svc> 
{
    let router = oles::router::Router::new();

    router
}

