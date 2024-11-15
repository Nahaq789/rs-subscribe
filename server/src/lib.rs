use aws_config::imds::Client;
use thiserror::Error;

#[derive(Debug)]
pub struct ApiSettings {
  pub host: String,
  pub port: String,
}

#[derive(Debug)]
pub struct AwsSettings {
  pub client: Client,
}

#[derive(Debug)]
pub struct AppSettings {
  pub api: ApiSettings,
  pub aws: AwsSettings,
}

#[derive(Debug, Error)]
pub enum SettingsError {
  #[error("Cannot load config. key: {0}")]
  InvalidLoadConfig(String),
}
