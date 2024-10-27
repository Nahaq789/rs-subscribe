use thiserror::Error;
use crate::value_object::amount::AmountError;

#[derive(Debug, Error)]
pub enum SubscribeError {
    #[error("Invalid Amount: {0:?}")]
    InvalidAmountError(#[from] AmountError)
}