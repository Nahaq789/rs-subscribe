use crate::value_object::amount::AmountError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SubscribeError {
    #[error("Invalid Amount: {0:?}")]
    InvalidAmountError(#[from] AmountError),
}
