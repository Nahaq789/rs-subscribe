use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::str::FromStr;

/// DynamoDBのデータをドメインモデルに変換するためのトレイトです
///
/// Tはドメインモデルの型、Eはエラー型を表します
///
/// # Example
/// ```
/// impl Mapper<PaymentMethod, PaymentError> for PaymentRepositoryImpl {
///     fn to_domain_model(v: HashMap<String, AttributeValue>) -> Result<PaymentMethod, PaymentError> {
///         // 実装
///     }
/// }
/// ```
pub trait Mapper<T, E> {
  fn map_to_domain_model(v: HashMap<String, AttributeValue>) -> Result<T, E>;
}

/// DynamoDBのAttributeValueから文字列を取得します
///
/// # Arguments
/// * `val` - DynamoDBのAttributeValue
/// * `default` - 値が取得できない場合のデフォルト値
///
/// # Returns
/// AttributeValueから取得した文字列、または指定されたデフォルト値
pub fn as_string(val: Option<&AttributeValue>, default: &str) -> String {
  val.and_then(|v| v.as_s().ok()).map(ToString::to_string).unwrap_or_else(|| default.to_string())
}

/// DynamoDBのAttributeValueからUTC日時を取得します
///
/// # Arguments
/// * `val` - DynamoDBのAttributeValue
///
/// # Returns
/// * `Some(DateTime<Utc>)` - 日時の解析に成功した場合
/// * `None` - 値が存在しない、または解析に失敗した場合
pub fn as_datetime(val: Option<&AttributeValue>) -> Option<DateTime<Utc>> {
  val.and_then(|v| v.as_s().ok()).and_then(|s| DateTime::<Utc>::from_str(s).ok())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_as_string_success() {
    let binding1 = &AttributeValue::S("hoge".to_string());
    let binding2 = &AttributeValue::S("fuga".to_string());
    let test_case = vec![
      (Some(binding1), "hoge".to_string()),
      (Some(binding2), "fuga".to_string()),
    ];

    for (found, expected) in test_case {
      let result = as_string(found, "");
      assert_eq!(result, expected)
    }
  }

  #[test]
  fn test_as_string_default_value() {
    let binding1 = None;
    let binding2 = None;
    let test_case = vec![
      (binding1, ""),
      (binding2, ""),
    ];

    for (found, expected) in test_case {
      let result = as_string(found, "");
      assert_eq!(result, expected)
    }
  }

  #[test]
  fn test_as_datetime_success() {
    let binding1 = AttributeValue::S(Utc::now().to_rfc3339());
    let result = as_datetime(Some(&binding1));

    assert!(result.is_some())
  }

  #[test]
  fn test_as_datetime_default_value() {
    let binding1 = None;
    let result = as_datetime(binding1);

    assert!(result.is_none())
  }
}
