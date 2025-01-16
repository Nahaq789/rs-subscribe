use std::{
  fmt::{Display, Formatter},
  str::FromStr,
};

use uuid::Uuid;

use crate::{generate_id, AggregateId, AggregateIdError};

/// 支払方法の一意識別子を表す構造体
///
/// フォーマット: "pay_<uuid>"
/// 例: "pay_550e8400-e29b-41d4-a716-446655440000"
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentMethodId {
  /// UUIDの文字列
  value: String,
}

const PAYMENT_PREFIX: &str = "pay";

impl PaymentMethodId {
  /// 新しい支払方法IDを生成する
  ///
  /// # 戻り値
  /// - [PaymentMethodId] 生成された支払方法ID
  pub fn new() -> Self {
    let value = generate_id(PAYMENT_PREFIX, None);
    Self { value }
  }
}

impl AggregateId for PaymentMethodId {
  /// プレフィックスを取得する
  ///
  /// # 戻り値
  /// - [String] "pay"という文字列
  fn type_name(&self) -> String {
    PAYMENT_PREFIX.to_string()
  }

  /// IDの値を取得する
  ///
  /// # 戻り値
  /// - [String] UUID文字列
  fn value(&self) -> &String {
    &self.value
  }
}

impl From<Uuid> for PaymentMethodId {
  /// UUIDから支払方法IDを生成する
  ///
  /// # 引数
  /// * `value` - [Uuid] 変換元のUUID
  ///
  /// # 戻り値
  /// - [PaymentMethodId] 生成された支払方法ID
  fn from(value: Uuid) -> Self {
    Self { value: generate_id(PAYMENT_PREFIX, Some(value)) }
  }
}

impl Display for PaymentMethodId {
  /// 文字列表現を取得する
  ///
  /// # 引数
  /// * `f` - [Formatter] フォーマッター
  ///
  /// # 戻り値
  /// - [std::fmt::Result] フォーマット結果
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.value)
  }
}

impl FromStr for PaymentMethodId {
  type Err = AggregateIdError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let value: Vec<&str> = s.split("_").collect();
    if value.len() != 2 {
      return Err(AggregateIdError::InvalidFormat);
    }
    if value[0] != PAYMENT_PREFIX {
      return Err(AggregateIdError::InvalidFormat);
    }
    let uuid = Uuid::parse_str(value[1]).map_err(|_| AggregateIdError::InvalidUuid)?;
    Ok(Self::from(uuid))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_success() {
    let id = PaymentMethodId::new();
    assert!(!id.value.is_empty());
    assert!(id.value.starts_with("pay_"));
  }

  #[test]
  fn test_type_name_success() {
    let id = PaymentMethodId::new();
    assert!(!id.type_name().is_empty());
    assert_eq!(PAYMENT_PREFIX, id.type_name());
  }

  #[test]
  fn test_from_success() {
    let uuid = Uuid::new_v4();
    let format = format!("pay_{}", uuid);
    let result = PaymentMethodId::from(uuid);
    assert!(result.value.starts_with("pay_"));
    assert_eq!(format, result.value);
  }

  #[test]
  fn test_from_str_success() {
    let uuid = Uuid::new_v4();
    let format = format!("pay_{}", uuid);
    let result = PaymentMethodId::from_str(&format);
    assert!(result.is_ok());
    assert_eq!(format, result.unwrap().value);
  }

  #[test]
  fn test_display_format() {
    let id = PaymentMethodId::new();
    let display_string = id.to_string();
    assert!(display_string.starts_with("pay_"));
  }

  #[test]
  fn test_clone_equality() {
    let id1 = PaymentMethodId::new();
    let id2 = id1.clone();
    assert_eq!(id1, id2);
  }

  #[test]
  fn test_debug_format() {
    let id = PaymentMethodId::new();
    let debug_string = format!("{:?}", id);
    assert!(!debug_string.is_empty());
  }

  #[test]
  fn test_from_str_failed_invalid_format() {
    let result = PaymentMethodId::from_str("invalid");
    assert!(matches!(result, Err(AggregateIdError::InvalidFormat)));
  }

  #[test]
  fn test_from_str_failed_invalid_uuid() {
    let format = format!("pay_{}", "invalid-uuid");
    let result = PaymentMethodId::from_str(&format);
    assert!(matches!(result, Err(AggregateIdError::InvalidUuid)));
  }

  #[test]
  fn test_from_str_failed_wrong_prefix() {
    let uuid = Uuid::new_v4();
    let format = format!("wrong_{}", uuid);
    let result = PaymentMethodId::from_str(&format);
    assert!(matches!(result, Err(AggregateIdError::InvalidFormat)));
  }

  #[test]
  fn test_value_reference() {
    let id = PaymentMethodId::new();
    let value_ref = id.value();
    assert_eq!(&id.value, value_ref);
  }

  #[test]
  fn test_multiple_instances_unique() {
    let id1 = PaymentMethodId::new();
    let id2 = PaymentMethodId::new();
    assert_ne!(id1, id2);
  }

  #[test]
  fn test_from_str_empty_string() {
    let result = PaymentMethodId::from_str("");
    assert!(matches!(result, Err(AggregateIdError::InvalidFormat)));
  }

  #[test]
  fn test_from_str_too_many_parts() {
    let result = PaymentMethodId::from_str("pay_uuid_extra");
    assert!(matches!(result, Err(AggregateIdError::InvalidFormat)));
  }

  #[test]
  fn test_display_and_from_str_consistency() {
    let original = PaymentMethodId::new();
    let display_string = original.to_string();
    assert!(display_string.contains(&original.value));
  }
}
