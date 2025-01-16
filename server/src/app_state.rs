use crate::app_state::StateError::BuildError;
use crate::client::{Database, DatabaseBuilder};
use application::service::payment_method_service::PaymentMethodServiceImpl;
use application::service::PaymentMethodService;
use infrastructure::repository_impl::payment_repository_impl::PaymentRepositoryImpl;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StateError {
  #[error("Invalid build failed: {0}")]
  BuildError(String),
}

pub type DynPaymentService = Arc<dyn PaymentMethodService + Send + Sync>;

#[derive(Clone)]
pub struct PaymentMethodState {
  pub state: DynPaymentService,
}

impl PaymentMethodState {
  pub async fn new(table: &str) -> Result<Self, StateError> {
    let build = Database::build(None).await;
    let client = match build {
      Ok(b) => b,
      Err(e) => return Err(BuildError(e.to_string())),
    };

    let repository = PaymentRepositoryImpl::new(client.client(), table);
    let service = PaymentMethodServiceImpl::new(repository);

    Ok(Self { state: Arc::new(service) })
  }
}
