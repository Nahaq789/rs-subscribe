use crate::app_state::StateError::BuildError;
use crate::client::{Database, DatabaseBuilder};
use application::service::category_service::CategoryServiceImpl;
use application::service::payment_method_service::PaymentMethodServiceImpl;
use application::service::subscribe_service::SubscribeServiceImpl;
use application::service::{CategoryService, PaymentMethodService, SubscribeService};
use infrastructure::repository_impl::category_repository_impl::CategoryRepositoryImpl;
use infrastructure::repository_impl::payment_repository_impl::PaymentRepositoryImpl;
use infrastructure::repository_impl::subscribe_repository_impl::SubscribeRepositoryImpl;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Invalid build failed: {0}")]
    BuildError(String),
}

pub type DynPaymentService = Arc<dyn PaymentMethodService + Send + Sync>;
pub type DynSubscribeService = Arc<dyn SubscribeService + Send + Sync>;
pub type DynCategoryService = Arc<dyn CategoryService + Send + Sync>;

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

#[derive(Clone)]
pub struct SubscribeState {
    pub state: DynSubscribeService,
}

impl SubscribeState {
    pub async fn new(table: &str) -> Result<Self, StateError> {
        let build = Database::build(None).await;
        let client = match build {
            Ok(b) => b,
            Err(e) => return Err(BuildError(e.to_string())),
        };

        let repository = SubscribeRepositoryImpl::new(client.client(), table);
        let service = SubscribeServiceImpl::new(repository);

        Ok(Self { state: Arc::new(service) })
    }
}

#[derive(Clone)]
pub struct CategoryState {
    pub state: DynCategoryService,
}

impl CategoryState {
    pub async fn new(table: &str) -> Result<Self, StateError> {
        let build = Database::build(None).await;
        let client = match build {
            Ok(b) => b,
            Err(e) => return Err(BuildError(e.to_string())),
        };

        let repository = CategoryRepositoryImpl::new(client.client(), table);
        let service = CategoryServiceImpl::new(repository);

        Ok(Self { state: Arc::new(service) })
    }
}
