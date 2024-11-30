use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum ApplicationError {
  #[error("Aggregate id format error: '{0}'")]
  InvalidAggregateIdFormatError(String),

  #[error("Payment method error: '{0}'")]
  PaymentMethodError(String),
}

#[cfg(test)]
mod tests {
  use crate::error::ApplicationError;
  use domain::payment::payment_error::PaymentError;
  use domain::payment::payment_method_name::PaymentMethodNameError;

  #[test]
  fn test_payment_method_error() {
    let error1 = PaymentMethodNameError::InvalidCategoryName("invalid category name".to_string());
    let error2 = PaymentError::PaymentMethodNameFailed(error1.to_string());

    let result = ApplicationError::PaymentMethodError(error2.to_string());
    println!("{}", result)
  }
}
