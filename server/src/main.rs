use axum::routing::get;
use axum::{Extension, Router};
use server::app_state::PaymentMethodState;
use server::controller::payment_method_controller::create_payment_method;
use std::net::{Ipv4Addr, SocketAddrV4};
use tracing::metadata::LevelFilter;
use tracing::{event, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, Layer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_ansi(false)
        .with_filter(
          filter::filter_fn(|metadata| {
            if metadata.target().contains("credentials") {
              false
            } else {
              true
            }
          })
        )
        .with_filter(LevelFilter::DEBUG)
    )
    .init();

  let app = create_router().await;
  let socket_addr_v4 = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080);
  let listener = tokio::net::TcpListener::bind(socket_addr_v4).await?;
  event!(Level::INFO, "Application Started");
  event!(Level::INFO, "Running on {0}", socket_addr_v4);
  axum::serve(listener, app).await?;

  Ok(())
}

async fn create_router() -> Router {
  let state = PaymentMethodState::new().await.unwrap();
  Router::new()
    .route("/", get(create_payment_method))
    .layer(Extension(state))
}
