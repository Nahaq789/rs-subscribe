use crate::dtos::payment_method_dto::PaymentMethodDTO;
use crate::dtos::DTO;
use crate::error::ApplicationError;
use crate::service::PaymentMethodService;
use domain::payment::payment_method_id::PaymentMethodId;
use domain::payment::PaymentMethod;
use domain::repository::payment_repository::PaymentRepository;
use domain::user::user_id::UserId;
use std::str::FromStr;

pub struct PaymentMethodServiceImpl<T: PaymentRepository> {
  repository: T,
}

impl<T: PaymentRepository> PaymentMethodServiceImpl<T> {
  pub fn new(repository: T) -> PaymentMethodServiceImpl<T> {
    Self { repository }
  }
}

#[async_trait::async_trait]
impl<T: PaymentRepository> PaymentMethodService for PaymentMethodServiceImpl<T> {
  async fn create_payment_method(&self, payment: PaymentMethodDTO) -> Result<(), ApplicationError> {
    let payment_method = PaymentMethodDTO::map_to_domain_model(payment)?;
    match self.repository.create(&payment_method).await {
      Ok(_) => Ok(()),
      Err(e) => Err(ApplicationError::PaymentMethodError(e.to_string())),
    }
  }

  async fn find_payment_method_all(
    &self,
    user_id: &str,
  ) -> Result<Vec<PaymentMethodDTO>, ApplicationError> {
    let user_id = UserId::from_str(user_id)
      .map_err(|e| ApplicationError::InvalidAggregateIdFormatError(e.to_string()))?;
    match self.repository.find_all(&user_id).await {
      Ok(v) => {
        let result = v
          .iter()
          .map(|item| PaymentMethodDTO::map_to_dto(item))
          .collect();
        Ok(result)
      }
      Err(e) => Err(ApplicationError::PaymentMethodError(e.to_string())),
    }
  }

  async fn find_payment_method_by_id(
    &self,
    payment_id: &str,
    user_id: &str,
  ) -> Result<PaymentMethodDTO, ApplicationError> {
    let payment_id = PaymentMethodId::from_str(payment_id)
      .map_err(|e| ApplicationError::InvalidAggregateIdFormatError(e.to_string()))?;
    let user_id = UserId::from_str(user_id)
      .map_err(|e| ApplicationError::InvalidAggregateIdFormatError(e.to_string()))?;
    match self.repository.find_by_id(&payment_id, &user_id).await {
      Ok(v) => {
        let result = PaymentMethodDTO::map_to_dto(&v);
        Ok(result)
      }
      Err(e) => Err(ApplicationError::PaymentMethodError(e.to_string())),
    }
  }

  async fn update_payment_method(&self, payment: PaymentMethodDTO) -> Result<(), ApplicationError> {
    let payment_method = PaymentMethodDTO::map_to_domain_model(payment)?;
    let exist = self
      .repository
      .exists(payment_method.payment_method_id(), payment_method.user_id())
      .await
      .map_err(|e| ApplicationError::PaymentMethodError(e.to_string()))?;

    PaymentMethod::exists(exist)
      .map_err(|e| ApplicationError::PaymentMethodError(e.to_string()))?;
    match self.repository.update(&payment_method).await {
      Ok(()) => Ok(()),
      Err(e) => Err(ApplicationError::PaymentMethodError(e.to_string())),
    }
  }

  async fn delete_payment_method(
    &self,
    payment_id: &str,
    user_id: &str,
  ) -> Result<(), ApplicationError> {
    let payment_id = PaymentMethodId::from_str(&payment_id)
      .map_err(|e| ApplicationError::InvalidAggregateIdFormatError(e.to_string()))?;
    let user_id = UserId::from_str(&user_id)
      .map_err(|e| ApplicationError::InvalidAggregateIdFormatError(e.to_string()))?;

    match self.repository.delete(&payment_id, &user_id).await {
      Ok(()) => Ok(()),
      Err(e) => Err(ApplicationError::PaymentMethodError(e.to_string())),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::Utc;
  use domain::payment::payment_error::PaymentError;
  use domain::payment::payment_method_name::{
    CreditCard, PaymentMethodCategoryName, PaymentMethodKindName,
  };
  use domain::payment::PaymentMethod;
  use domain::AggregateId;
  use mockall::mock;

  mock! {
    PaymentRepository {}
    #[async_trait::async_trait]
    impl PaymentRepository for PaymentRepository {
      async fn create(&self, payment: &PaymentMethod) -> Result<(), PaymentError>;
      async fn find_all(&self, user_id: &UserId) -> Result<Vec<PaymentMethod>, PaymentError>;
      async fn find_by_id(&self, payment_id: &PaymentMethodId, user_id: &UserId) -> Result<PaymentMethod, PaymentError>;
      async fn update(&self, payment: &PaymentMethod) -> Result<(), PaymentError>;
      async fn delete(&self, payment_id: &PaymentMethodId, user_id: &UserId) -> Result<(), PaymentError>;
      async fn exists(&self, payment_id: &PaymentMethodId, user_id: &UserId) -> Result<bool, PaymentError>;
    }
  }

  fn create_mock_dto() -> PaymentMethodDTO {
    let dto = PaymentMethodDTO {
      payment_method_id: PaymentMethodId::new().value().to_string(),
      user_id: UserId::new().value().to_string(),
      method_name: "Credit Card".to_string(),
      method_kind_name: "JCB".to_string(),
      additional_name: "test_card".to_string(),
      created_at: Utc::now(),
      updated_at: None,
    };
    dto
  }

  fn create_mock_payment_domain() -> PaymentMethod {
    let domain = PaymentMethod::new(
      PaymentMethodId::new(),
      UserId::new(),
      PaymentMethodCategoryName::CreditCard,
      PaymentMethodKindName::CreditCard(CreditCard::JCB),
      "hoge",
      Utc::now(),
      None,
    );
    domain
  }
  #[tokio::test]
  async fn test_create_payment_method() {
    let mut mock_repository = MockPaymentRepository::new();

    mock_repository
      .expect_create()
      .return_once(move |_| Ok(()))
      .times(1);

    let dto = create_mock_dto();

    let payment_service = PaymentMethodServiceImpl::new(mock_repository);

    let result = payment_service.create_payment_method(dto).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ())
  }

  #[tokio::test]
  async fn test_create_payment_method_failed() {
    let mut mock_repository = MockPaymentRepository::new();
    mock_repository
      .expect_create()
      .return_once(move |_| Err(PaymentError::CreatePaymentMethodFailed("hoge".to_string())))
      .times(1);

    let dto = create_mock_dto();

    let payment_service = PaymentMethodServiceImpl::new(mock_repository);

    let result = payment_service.create_payment_method(dto).await;

    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err(),
      ApplicationError::PaymentMethodError("Failed to create payment method: hoge".to_string())
    )
  }

  #[tokio::test]
  async fn test_find_payment_method_all() {
    let mut mock_repository = MockPaymentRepository::new();
    mock_repository
      .expect_find_all()
      .return_once(move |_| {
        let mut vec: Vec<PaymentMethod> = vec![];
        vec.push(create_mock_payment_domain());
        Ok(vec)
      })
      .times(1);

    let payment_service = PaymentMethodServiceImpl::new(mock_repository);
    let user_id = UserId::new();
    let result = payment_service
      .find_payment_method_all(user_id.value())
      .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
  }

  #[tokio::test]
  async fn test_find_payment_method_all_failed() {
    let mut mock_repository = MockPaymentRepository::new();
    mock_repository
      .expect_find_all()
      .return_once(move |_| Err(PaymentError::QueryError("hoge".to_string())))
      .times(1);

    let payment_service = PaymentMethodServiceImpl::new(mock_repository);
    let user_id = UserId::new();
    let result = payment_service
      .find_payment_method_all(user_id.value())
      .await;

    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err(),
      ApplicationError::PaymentMethodError("Failed to query payment methods: hoge".to_string())
    )
  }

  #[tokio::test]
  async fn test_find_payment_method_by_id() {
    let mut mock_repository = MockPaymentRepository::new();
    mock_repository
      .expect_find_by_id()
      .return_once(move |_, _| {
        let domain = create_mock_payment_domain();
        Ok(domain)
      })
      .times(1);

    let payment_service = PaymentMethodServiceImpl::new(mock_repository);
    let user_id = UserId::new();
    let payment_id = PaymentMethodId::new();

    let result = payment_service
      .find_payment_method_by_id(payment_id.value(), user_id.value())
      .await;
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_find_payment_method_by_id_failed() {
    let mut mock_repository = MockPaymentRepository::new();
    mock_repository
      .expect_find_by_id()
      .return_once(move |_, _| Err(PaymentError::FindByIdError("hoge".to_string())))
      .times(1);

    let payment_service = PaymentMethodServiceImpl::new(mock_repository);
    let user_id = UserId::new();
    let payment_id = PaymentMethodId::new();

    let result = payment_service
      .find_payment_method_by_id(payment_id.value(), user_id.value())
      .await;

    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err(),
      ApplicationError::PaymentMethodError(
        "Failed to find by id payment methods: hoge".to_string()
      )
    )
  }

  #[tokio::test]
  async fn test_update_payment_method() {
    let mut mock_repository = MockPaymentRepository::new();
    mock_repository
      .expect_update()
      .return_once(move |_| Ok(()))
      .times(1);

    mock_repository
      .expect_exists()
      .return_once(move |_, _| Ok(true))
      .times(1);

    let dto = create_mock_dto();
    let payment_service = PaymentMethodServiceImpl::new(mock_repository);
    let result = payment_service.update_payment_method(dto).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ());
  }

  #[tokio::test]
  async fn test_update_payment_method_failed() {
    let mut mock_repository = MockPaymentRepository::new();
    mock_repository
      .expect_update()
      .return_once(move |_| Err(PaymentError::UpdatePaymentMethodError("hoge".to_string())))
      .times(1);

    mock_repository
      .expect_exists()
      .return_once(move |_, _| Ok(true));

    let dto = create_mock_dto();
    let payment_service = PaymentMethodServiceImpl::new(mock_repository);
    let result = payment_service.update_payment_method(dto).await;

    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err(),
      ApplicationError::PaymentMethodError("Failed to update payment method: hoge".to_string())
    );
  }

  #[tokio::test]
  async fn test_delete_payment_method() {
    let mut mock_repository = MockPaymentRepository::new();
    mock_repository
      .expect_delete()
      .return_once(move |_, _| Ok(()))
      .times(1);

    let payment_service = PaymentMethodServiceImpl::new(mock_repository);
    let user_id = UserId::new();
    let payment_id = PaymentMethodId::new();

    let result = payment_service
      .delete_payment_method(payment_id.value(), user_id.value())
      .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ());
  }

  #[tokio::test]
  async fn test_delete_payment_method_failed() {
    let mut mock_repository = MockPaymentRepository::new();
    mock_repository
      .expect_delete()
      .return_once(move |_, _| Err(PaymentError::DeletePaymentMethodFailed("hoge".to_string())))
      .times(1);

    let payment_service = PaymentMethodServiceImpl::new(mock_repository);
    let user_id = UserId::new();
    let payment_id = PaymentMethodId::new();

    let result = payment_service
      .delete_payment_method(payment_id.value(), user_id.value())
      .await;

    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err(),
      ApplicationError::PaymentMethodError("Failed to delete payment method: hoge".to_string())
    );
  }
}
