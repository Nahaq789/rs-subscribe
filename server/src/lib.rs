pub mod app_state;
pub mod client;
pub mod controller;

use std::str::FromStr;
use thiserror::Error;
use tracing::error;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, EnvFilter, Layer};

#[derive(Debug)]
pub struct ApiSettings {
  pub host: String,
  pub port: String,
}

#[derive(Debug, Error)]
pub enum SettingsError {
  #[error("Cannot load env. key: {0}")]
  InvalidLoadConfig(String),
}

impl ApiSettings {
  pub fn build() -> Result<Self, SettingsError> {
    let host = std::env::var("HOST")
      .map_err(|_| SettingsError::InvalidLoadConfig("HOST".to_string()))?;
    let port = std::env::var("PORT")
      .map_err(|_| SettingsError::InvalidLoadConfig("PORT".to_string()))?;

    Ok(Self { host, port })
  }
}

pub fn set_up_tracing_subscriber() {
  const CREDENTIALS: &str = "credentials";
  let filter = EnvFilter::from_default_env();

  tracing_subscriber::registry()
    .with(
      tracing_subscriber::fmt::layer()
        .json()
        .with_target(true)
        .with_ansi(false)
        .with_filter(filter)
        .with_filter(filter::filter_fn(|metadata| {
          if metadata.target().contains(CREDENTIALS) {
            false
          } else {
            true
          }
        })),
    )
    .init();
}
