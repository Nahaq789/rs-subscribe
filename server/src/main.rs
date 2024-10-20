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
  let addr = "0.0.0.0:8080";
  let listener = tokio::net::TcpListener::bind(addr).await?;
  event!(Level::INFO, "Application Started");
  axum::serve(listener, app).await?;

  Ok(())
}

async fn create_router() -> Router {
  Router::new()
    .route("/", get({ "aaa" }))
}
