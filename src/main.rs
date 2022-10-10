pub mod server;
pub mod handler;
pub mod router;
pub mod response;

use std::net::SocketAddr;
use std::net::Ipv4Addr;

use server::Server;

#[tokio::main]
async fn main() {
    let serv = Server::new(SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000)).await;
    serv.run(server::make_service).await;
}