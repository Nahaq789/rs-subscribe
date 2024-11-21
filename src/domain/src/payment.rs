pub mod payment_error;
pub mod payment_method;
pub mod payment_method_id;

use chrono::{DateTime, Utc};
use payment_method::PaymentMethodName;
use payment_method_id::PaymentMethodId;

/// 支払方法情報を管理する構造体
///
/// # フィールド
/// * `payment_method_id` - [PaymentMethodId] 支払方法を一意に識別するID
/// * `method_name` - [PaymentMethodName] 支払方法名（クレジットカード、口座振替など）
/// * `additional_name` - 追加の支払方法名（任意）
/// * `created_at` - 作成日時
/// * `updated_at` - 更新日時
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PaymentMethod {
  payment_method_id: PaymentMethodId,
  method_name: PaymentMethodName,
  method_kind_name: String,
  additional_name: String,
  created_at: Option<DateTime<Utc>>,
  updated_at: Option<DateTime<Utc>>,
}

impl PaymentMethod {
  /// 新しい支払方法インスタンスを作成する
  ///
  /// # 引数
  /// * `payment_method_id` - [PaymentMethodId] 支払方法ID
  /// * `method_name` - [PaymentMethodName] 支払方法名
  /// * `detail_name` - [DetailMethodName] 詳細な支払方法名
  /// * `additional_name` - 追加の支払方法名（オプション）
  /// * `created_at` - 作成日時（オプション）
  /// * `updated_at` - 更新日時（オプション）
  ///
  /// # 戻り値
  /// - [PaymentMethod] 作成された支払方法情報
  pub fn new(
    payment_method_id: PaymentMethodId,
    method_name: PaymentMethodName,
    additional_name: &str,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
  ) -> Self {
    Self {
      payment_method_id,
      method_name: method_name.clone(),
      method_kind_name: method_name.to_string(),
      additional_name: additional_name.to_string(),
      created_at,
      updated_at,
    }
  }

  pub fn payment_method_id(&self) -> &PaymentMethodId {
    &self.payment_method_id
  }

  pub fn method_name(&self) -> &PaymentMethodName {
    &self.method_name
  }

  pub fn additional_name(&self) -> &String {
    &self.additional_name
  }

  pub fn created_at(&self) -> &Option<DateTime<Utc>> {
    &self.created_at
  }

  pub fn updated_at(&self) -> &Option<DateTime<Utc>> {
    &self.updated_at
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_payment_method_new() {
    let payment_method_id = PaymentMethodId::new();
    let method_name = PaymentMethodName::from_str("Visa").unwrap();
    let additional_name = "hoge";
    let created_at = Some(Utc::now());
    let updated_at = Some(Utc::now());

    let result = PaymentMethod::new(
      payment_method_id.clone(),
      method_name.clone(),
      additional_name,
      created_at,
      updated_at,
    );

    assert_eq!(payment_method_id, result.payment_method_id);
    assert_eq!(method_name, result.method_name);
    assert_eq!(additional_name, result.additional_name);
    assert!(created_at.is_some());
    assert_eq!(created_at, result.created_at);
    assert!(updated_at.is_some());
    assert_eq!(updated_at, result.updated_at);
  }
}
