use crate::{payment_cycle::PaymentCycleError, value_object::amount::AmountError, AggregateIdError};
use thiserror::Error;

use super::subscribe_name::SubscribeNameError;

/// サブスクリプション操作に関するエラー
///
/// # バリアント
/// * `InvalidAmountError` - 金額が不正な場合のエラー
///
/// # 実装
/// * `thiserror::Error` を導出して、エラーメッセージを定義
/// * `AmountError` からの変換を実装
#[derive(Debug, Error)]
pub enum SubscribeError {
  #[error("Invalid Amount: {0:?}")]
  InvalidAmountError(#[from] AmountError),

  #[error("Not match Subscribe Status: {0}")]
  InvalidSubscribeStatus(String),

  #[error("Failed to delete subscribe: {0}")]
  DeleteSubscribeFailed(String),

  #[error("Failed to query subscribe: {0}")]
  QueryError(String),

  #[error("Failed to find by id subscribe: {0}")]
  FindByIdError(String),

  #[error("Failed to update subscribe: {0}")]
  UpdateSubscribeError(String),

  #[error("Required subscribe field '{0}' was missing")]
  MissingField(String),

  #[error("Failed to create subscribe: {0}")]
  CreateSubscribeFailed(String),

  #[error("Failed to parse field '{0}'")]
  ParseFailed(String),

  #[error("Subscribe not exist")]
  NotExists,

  #[error("{0}")]
  SubscribeIdFailed(String),

  #[error("{0}")]
  InvalidPaymentCycle(String),

  #[error("{0}")]
  InvalidSubscribeName(String),
}

impl From<AggregateIdError> for SubscribeError {
  fn from(value: AggregateIdError) -> Self {
    SubscribeError::SubscribeIdFailed(value.to_string())
  }
}

impl From<PaymentCycleError> for SubscribeError {
  fn from(value: PaymentCycleError) -> Self {
    SubscribeError::InvalidPaymentCycle(value.to_string())
  }
}

impl From<SubscribeNameError> for SubscribeError {
  fn from(value: SubscribeNameError) -> Self {
    SubscribeError::InvalidSubscribeName(value.to_string())
  }
}
