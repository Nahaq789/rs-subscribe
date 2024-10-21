use std::net::{Ipv6Addr, SocketAddrV6};
use axum::Router;
use axum::routing::get;
use tracing::{event, Level};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt()
    .with_max_level(Level::DEBUG)
    .with_ansi(false)
    .init();

  let app = create_router().await;
  let socket_addr_v6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 8080, 0, 0);
  let listener = tokio::net::TcpListener::bind(socket_addr_v6).await?;
  event!(Level::INFO, "Application Started");
  axum::serve(listener, app).await?;

  Ok(())
}

async fn create_router() -> Router {
  Router::new()
    .route("/", get({ "hoge" }))
}
