use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaymentError {
  #[error("Failed to create payment method: {0}")]
  CreatePaymentMethodFailed(String),

  #[error("Failed to delete payment method: {0}")]
  DeletePaymentMethodFailed(String),
}
