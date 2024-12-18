use crate::{value_object::amount::AmountError, AggregateIdError};
use thiserror::Error;

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

  #[error("Subscribe not exist")]
  NotExists,

  #[error("{0}")]
  SubscribeIdFailed(String),
}

impl From<AggregateIdError> for SubscribeError {
  fn from(value: AggregateIdError) -> Self {
    SubscribeError::SubscribeIdFailed(value.to_string())
  }
}
