use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaymentError {
  #[error("Invalid delete payment payment item: {0}")]
  InvalidDeleteItem(String),
}
