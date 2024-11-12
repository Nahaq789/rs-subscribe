use crate::value_object::amount::AmountError;
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
}
