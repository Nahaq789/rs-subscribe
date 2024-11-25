use crate::payment::payment_method_name::PaymentMethodNameError;
use crate::AggregateIdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaymentError {
  #[error("Failed to create payment method: {0}")]
  CreatePaymentMethodFailed(String),

  #[error("Failed to delete payment method: {0}")]
  DeletePaymentMethodFailed(String),

  #[error("Failed to query payment methods: {0}")]
  QueryError(String),

  #[error("Failed to find by id payment methods: {0}")]
  FindByIdError(String),

  #[error("Failed to update payment method: {0}")]
  UpdatePaymentMethodError(String),

  #[error("{0}")]
  PaymentMethodIdFailed(String),

  #[error("{0}")]
  PaymentMethodNameFailed(String),
}

impl From<AggregateIdError> for PaymentError {
  fn from(value: AggregateIdError) -> Self {
    PaymentError::PaymentMethodIdFailed(value.to_string())
  }
}

impl From<PaymentMethodNameError> for PaymentError {
  fn from(value: PaymentMethodNameError) -> Self {
    PaymentError::PaymentMethodNameFailed(value.to_string())
  }
}
