use domain::{payment::payment_error::PaymentError, subscribe::subscribe_error::SubscribeError, AggregateIdError};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum ApplicationError {
    #[error("Aggregate id format error: '{0}'")]
    InvalidAggregateIdFormatError(String),

    #[error("Payment method error: '{0}'")]
    PaymentMethodError(String),

    #[error("Subscribe error: '{0}")]
    SubscribeError(String),
}
impl From<PaymentError> for ApplicationError {
    fn from(value: PaymentError) -> Self {
        let error = Self::PaymentMethodError(value.to_string());
        error
    }
}

impl From<AggregateIdError> for ApplicationError {
    fn from(value: AggregateIdError) -> Self {
        let error = Self::InvalidAggregateIdFormatError(value.to_string());
        error
    }
}

impl From<SubscribeError> for ApplicationError {
    fn from(value: SubscribeError) -> Self {
        let error = Self::SubscribeError(value.to_string());
        error
    }
}

pub fn to_aggregate_id_error<E: ToString>(e: E) -> ApplicationError {
    ApplicationError::InvalidAggregateIdFormatError(e.to_string())
}

pub fn to_payment_method_error<E: ToString>(e: E) -> ApplicationError {
    ApplicationError::PaymentMethodError(e.to_string())
}

pub fn to_subscribe_error<E: ToString>(e: E) -> ApplicationError {
    ApplicationError::SubscribeError(e.to_string())
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
