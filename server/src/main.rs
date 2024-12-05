use axum::routing::get;
use axum::{Extension, Router};
use dotenv::dotenv;
use server::app_state::PaymentMethodState;
use server::controller::payment_method_controller::create_payment_method;
use server::{set_up_tracing_subscriber, ApiSettings};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;
use tracing::{error, event, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, Layer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().ok();
  set_up_tracing_subscriber();

  let app = create_router().await;
  let api = ApiSettings::build().map_err(|e| {
    error!("{}", e);
    e
  })?;
  let socket_addr_v4 = SocketAddrV4::new(Ipv4Addr::from_str(&api.host)?, api.port.parse()?);
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
