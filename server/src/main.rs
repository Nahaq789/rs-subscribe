use axum::routing::get;
use axum::Router;
use std::net::{Ipv4Addr, SocketAddrV4};
use tracing::{event, Level};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_ansi(false)
        .init();

    let app = create_router().await;
    let socket_addr_v4 = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080);
    let listener = tokio::net::TcpListener::bind(socket_addr_v4).await?;
    event!(Level::INFO, "Application Started");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn create_router() -> Router {
    Router::new().route("/", get("hoge"))
}
