use aws_config::{meta::region::RegionProviderChain, Region};
use aws_sdk_dynamodb::Client;
use axum::async_trait;

#[async_trait]
pub trait DatabaseBuilder {
  type Client;
  type BuilderError;

  async fn build(conn: Option<String>) -> Result<Database<Self::Client>, Self::BuilderError>
  where
    Self: Sized;
}

pub struct Database<T> {
  client: T,
}

impl<T> Database<T> {
  pub fn client(&self) -> &T {
    &self.client
  }
}

#[async_trait]
impl DatabaseBuilder for Database<aws_sdk_dynamodb::Client> {
  type Client = aws_sdk_dynamodb::Client;
  type BuilderError = aws_sdk_dynamodb::Error;

  async fn build(_: Option<String>) -> Result<Database<Self::Client>, Self::BuilderError> {
    let region = std::env::var("AWS_REGION").unwrap_or_else(|_| "ap-northeast-1".to_string());
    let region_provider = RegionProviderChain::first_try(Region::new(region)).or_default_provider();

    let shared_config = aws_config::from_env().region(region_provider).load().await;

    let client = Client::new(&shared_config);

    Ok(Database { client })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_database_client_build_dynamodb() {
    let result = Database::<aws_sdk_dynamodb::Client>::build(None).await;
    assert!(result.is_ok());

    let client = result.unwrap().client;
    let region = client.config();

    assert_eq!(
      &aws_config::Region::new("ap-northeast-1"),
      region.region().unwrap()
    );
  }
}
