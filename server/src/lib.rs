pub mod app_state;
pub mod client;
pub mod controller;

use app_state::{PaymentMethodState, SubscribeState};
use axum::routing::{delete, get, patch, post};
use axum::{Extension, Router};
use controller::payment_method_controller::{
    create_payment_method, delete_payment_method, find_payment_method_all, find_payment_method_by_id,
    update_payment_method,
};
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

#[derive(Debug)]
pub struct AwsSettings {
    payment: String,
    subscribe: String,
    category: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SettingsError {
    #[error("Cannot load env. key: {0}")]
    InvalidLoadConfig(String),

    #[error("State build error: {0}")]
    StateBuildError(String),
}

impl ApiSettings {
    pub fn build() -> Result<Self, SettingsError> {
        let host = std::env::var("HOST").map_err(|_| SettingsError::InvalidLoadConfig("HOST".to_string()))?;
        let port = std::env::var("PORT").map_err(|_| SettingsError::InvalidLoadConfig("PORT".to_string()))?;

        Ok(Self { host, port })
    }
}

impl AwsSettings {
    pub fn build() -> Result<Self, SettingsError> {
        let payment = std::env::var("PAYMENT_TABLE")
            .map_err(|_| SettingsError::InvalidLoadConfig("PAYMENT_TABLE".to_string()))?;
        let subscribe = std::env::var("SUBSCRIBE_TABLE")
            .map_err(|_| SettingsError::InvalidLoadConfig("SUBSCRIBE_TABLE".to_string()))?;
        let category = std::env::var("CATEGORY_TABLE")
            .map_err(|_| SettingsError::InvalidLoadConfig("CATEGORY_TABLE".to_string()))?;

        Ok(Self { payment, subscribe, category })
    }
}

pub fn set_up_tracing_subscriber() {
    const CREDENTIALS: &str = "credentials";
    let filter = EnvFilter::from_default_env();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer().json().with_target(true).with_ansi(false).with_filter(filter).with_filter(
                filter::filter_fn(|metadata| if metadata.target().contains(CREDENTIALS) { false } else { true }),
            ),
        )
        .init();
}

pub async fn create_payment_router() -> Result<Router, SettingsError> {
    let aws = AwsSettings::build()?;
    let state =
        PaymentMethodState::new(&aws.payment).await.map_err(|e| SettingsError::StateBuildError(e.to_string()))?;
    Ok(Router::new()
        .route("/payment/create", post(create_payment_method))
        .route("/payment", get(find_payment_method_all))
        .route("/payment/id", get(find_payment_method_by_id))
        .route("/payment/update", patch(update_payment_method))
        .route("/payment/delete", delete(delete_payment_method))
        .layer(Extension(state)))
}

pub async fn create_subscribe_router() -> Result<Router, SettingsError> {
    let aws = AwsSettings::build()?;
    let state = SubscribeState::new(&aws.subscribe).await.map_err(|e| SettingsError::StateBuildError(e.to_string()))?;
    Ok(Router::new()
        .route("/subscribe/create", post(create_subscribe))
        .route("/subscribe", get(find_subscribe_all))
        .route("/subscribe/id", get(find_subscribe_by_id))
        .route("/subscribe/update", patch(update_subscribe))
        .route("/subscribe/delete", delete(delete_subscribe))
        .layer(Extension(state)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOST: &str = "1.1.1.1";
    const PORT: &str = "7878";
    const PAYMENT_TABLE: &str = "payment";

    fn clear_env() {
        std::env::remove_var("HOST");
        std::env::remove_var("PORT");
        std::env::remove_var("PAYMENT_TABLE");
    }

    #[test]
    fn test_api_settings_build_success() {
        clear_env();
        std::env::set_var("HOST", HOST);
        std::env::set_var("PORT", PORT);

        let result = ApiSettings::build();

        assert!(&result.is_ok());
        let result = result.unwrap();
        assert_eq!(&result.host, HOST);
        assert_eq!(&result.port, PORT)
    }

    #[test]
    fn test_api_settings_build_host_failed() {
        clear_env();
        let result = ApiSettings::build();

        assert!(result.is_err());
        assert_eq!(SettingsError::InvalidLoadConfig("HOST".to_string()), result.unwrap_err())
    }

    #[test]
    fn test_api_settings_build_port_failed() {
        clear_env();
        std::env::set_var("HOST", HOST);
        let result = ApiSettings::build();

        assert!(result.is_err());
        assert_eq!(SettingsError::InvalidLoadConfig("PORT".to_string()), result.unwrap_err())
    }

    #[test]
    fn aws_settings_build_payment_success() {
        clear_env();
        std::env::set_var("PAYMENT_TABLE", PAYMENT_TABLE);
        let result = AwsSettings::build();

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(&result.payment, PAYMENT_TABLE)
    }

    #[test]
    fn aws_settings_build_payment_failed() {
        clear_env();
        let result = AwsSettings::build();

        assert!(result.is_err());
        assert_eq!(SettingsError::InvalidLoadConfig("PAYMENT_TABLE".to_string()), result.unwrap_err())
    }

    #[tokio::test]
    async fn test_create_payment_router() {
        clear_env();
        std::env::set_var("PAYMENT_TABLE", PAYMENT_TABLE);
        let result = create_payment_router().await;
        assert!(result.is_ok())
    }
}
