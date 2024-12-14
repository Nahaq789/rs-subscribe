pub mod payment_error;
pub mod payment_method_id;
pub mod payment_method_name;

use std::str::FromStr;

use crate::payment::payment_error::PaymentError;
use crate::payment::payment_method_name::PaymentMethodCategoryName;
use crate::user::user_id::UserId;
use chrono::{DateTime, Utc};
use payment_method_id::PaymentMethodId;
use payment_method_name::PaymentMethodKindName;

/// 支払方法情報を管理する構造体
///
/// # フィールド
/// * `payment_method_id` - [PaymentMethodId] 支払方法を一意に識別するID
/// * `user_id` - [UserId] ユーザーID
/// * `method_name` - [PaymentMethodCategoryName] 支払方法名（クレジットカード、口座振替など）
/// * `method_kind_name` - [PaymentMethodKindName] 支払方法種類名（Visa、PayPalなど）
/// * `additional_name` - 追加の支払方法名（任意）
/// * `created_at` - 作成日時
/// * `updated_at` - 更新日時
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PaymentMethod {
  payment_method_id: PaymentMethodId,
  user_id: UserId,
  method_name: PaymentMethodCategoryName,
  method_kind_name: PaymentMethodKindName,
  additional_name: String,
  created_at: DateTime<Utc>,
  updated_at: Option<DateTime<Utc>>,
}

impl PaymentMethod {
  /// 新しい支払方法インスタンスを作成する
  ///
  /// # 引数
  /// * `payment_method_id` - [PaymentMethodId] 支払方法ID
  /// * `user_id` - [UserId] ユーザーID
  /// * `method_name` - [PaymentMethodKindName] 支払方法名
  /// * `detail_name` - [DetailMethodName] 詳細な支払方法名
  /// * `additional_name` - 追加の支払方法名（オプション）
  /// * `created_at` - 作成日時
  /// * `updated_at` - 更新日時（オプション）
  ///
  /// # 戻り値
  /// - [PaymentMethod] 作成された支払方法情報
  pub fn new(
    payment_method_id: PaymentMethodId,
    user_id: UserId,
    method_name: PaymentMethodCategoryName,
    method_kind_name: PaymentMethodKindName,
    additional_name: &str,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
  ) -> Self {
    Self {
      payment_method_id,
      user_id,
      method_name,
      method_kind_name,
      additional_name: additional_name.to_string(),
      created_at,
      updated_at,
    }
  }

  pub fn payment_method_id(&self) -> &PaymentMethodId {
    &self.payment_method_id
  }

  pub fn user_id(&self) -> &UserId {
    &self.user_id
  }

  pub fn method_name(&self) -> &PaymentMethodCategoryName {
    &self.method_name
  }

  pub fn method_kind_name(&self) -> &PaymentMethodKindName {
    &self.method_kind_name
  }

  pub fn additional_name(&self) -> &String {
    &self.additional_name
  }

  pub fn created_at(&self) -> &DateTime<Utc> {
    &self.created_at
  }

  pub fn updated_at(&self) -> &Option<DateTime<Utc>> {
    &self.updated_at
  }

  fn _is_valid_method_combination(
    method_name: &PaymentMethodCategoryName,
    kind_name: &PaymentMethodKindName,
  ) -> bool {
    match kind_name {
      PaymentMethodKindName::CreditCard(_) => method_name == &PaymentMethodCategoryName::CreditCard,
      PaymentMethodKindName::DigitalMoney(_) => {
        method_name == &PaymentMethodCategoryName::DigitalMoney
      }
      PaymentMethodKindName::MobilePayment(_) => {
        method_name == &PaymentMethodCategoryName::MobilePayment
      }
      PaymentMethodKindName::DigitalWallet(_) => {
        method_name == &PaymentMethodCategoryName::DigitalWallet
      }
      PaymentMethodKindName::BankTransfer(_) => {
        method_name == &PaymentMethodCategoryName::BankTransfer
      }
      PaymentMethodKindName::BNPL(_) => method_name == &PaymentMethodCategoryName::BNPL,
      PaymentMethodKindName::DebitCard => method_name == &PaymentMethodCategoryName::DebitCard,
      PaymentMethodKindName::CarrierBilling => {
        method_name == &PaymentMethodCategoryName::CarrierBilling
      }
    }
  }

  pub fn is_valid_method_combination(
    method_name: &PaymentMethodCategoryName,
    kind_name: &PaymentMethodKindName,
  ) -> Result<(), PaymentError> {
    match Self::_is_valid_method_combination(method_name, kind_name) {
      true => Ok(()),
      false => Err(PaymentError::InvalidMethodCombination),
    }
  }

  pub fn exists(exist: bool) -> Result<(), PaymentError> {
    match exist {
      true => Ok(()),
      false => Err(PaymentError::NotExists),
    }
  }

  pub fn make_updated_at(created_at: &Option<DateTime<Utc>>) -> Option<DateTime<Utc>> {
    match created_at {
      Some(_) => Some(Utc::now()),
      None => None,
    }
  }

  pub fn make_created_at(
    payment_id: &str,
    created_at: Option<DateTime<Utc>>,
  ) -> Result<DateTime<Utc>, PaymentError> {
    if "".eq(payment_id) || created_at.is_none() {
      return Ok(Utc::now());
    }
    match DateTime::from_str(&created_at.unwrap_or_default().to_string()) {
      Ok(v) => Ok(v),
      Err(e) => Err(PaymentError::InvalidFormatDatetime(e.to_string())),
    }
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  use crate::payment::payment_method_name::{
    BankTransfer, CreditCard, DigitalMoney, DigitalWallet, MobilePayment, BNPL,
  };

  #[test]
  fn test_payment_method_new() {
    let payment_method_id = PaymentMethodId::new();
    let user_id = UserId::new();
    let method_name = PaymentMethodCategoryName::CreditCard;
    let method_kind_name = PaymentMethodKindName::CreditCard(CreditCard::Visa);
    let additional_name = "hoge";
    let created_at = Utc::now();
    let updated_at = Some(Utc::now());

    let result = PaymentMethod::new(
      payment_method_id.clone(),
      user_id.clone(),
      method_name.clone(),
      method_kind_name.clone(),
      additional_name,
      created_at,
      updated_at,
    );

    assert_eq!(payment_method_id, result.payment_method_id);
    assert_eq!(user_id, result.user_id);
    assert_eq!(method_name, result.method_name);
    assert_eq!(method_kind_name, result.method_kind_name);
    assert_eq!(additional_name, result.additional_name);
    assert_eq!(created_at, result.created_at);
    assert!(updated_at.is_some());
    assert_eq!(updated_at, result.updated_at);
  }

  #[test]
  fn test_is_valid_method_combination_return_true() {
    let test_case = vec![
      (
        PaymentMethodCategoryName::CreditCard,
        PaymentMethodKindName::CreditCard(CreditCard::JCB),
      ),
      (
        PaymentMethodCategoryName::DigitalMoney,
        PaymentMethodKindName::DigitalMoney(DigitalMoney::Nanaco),
      ),
      (
        PaymentMethodCategoryName::MobilePayment,
        PaymentMethodKindName::MobilePayment(MobilePayment::DBarai),
      ),
      (
        PaymentMethodCategoryName::DigitalWallet,
        PaymentMethodKindName::DigitalWallet(DigitalWallet::ApplePay),
      ),
      (
        PaymentMethodCategoryName::BankTransfer,
        PaymentMethodKindName::BankTransfer(BankTransfer::JapaneseBankTransfer),
      ),
      (
        PaymentMethodCategoryName::BNPL,
        PaymentMethodKindName::BNPL(BNPL::Affirm),
      ),
      (
        PaymentMethodCategoryName::DebitCard,
        PaymentMethodKindName::DebitCard,
      ),
      (
        PaymentMethodCategoryName::CarrierBilling,
        PaymentMethodKindName::CarrierBilling,
      ),
    ];

    for (a, b) in test_case {
      assert!(PaymentMethod::_is_valid_method_combination(&a, &b))
    }
  }

  #[test]
  fn test_is_valid_method_combination_return_false() {
    let test_case = vec![
      (
        PaymentMethodCategoryName::CreditCard,
        PaymentMethodKindName::DigitalMoney(DigitalMoney::Nanaco),
      ),
      (
        PaymentMethodCategoryName::DigitalMoney,
        PaymentMethodKindName::MobilePayment(MobilePayment::DBarai),
      ),
      (
        PaymentMethodCategoryName::MobilePayment,
        PaymentMethodKindName::DigitalWallet(DigitalWallet::ApplePay),
      ),
      (
        PaymentMethodCategoryName::DigitalWallet,
        PaymentMethodKindName::BankTransfer(BankTransfer::JapaneseBankTransfer),
      ),
      (
        PaymentMethodCategoryName::BankTransfer,
        PaymentMethodKindName::BNPL(BNPL::Affirm),
      ),
      (
        PaymentMethodCategoryName::BNPL,
        PaymentMethodKindName::DebitCard,
      ),
      (
        PaymentMethodCategoryName::DebitCard,
        PaymentMethodKindName::CarrierBilling,
      ),
      (
        PaymentMethodCategoryName::CarrierBilling,
        PaymentMethodKindName::CreditCard(CreditCard::JCB),
      ),
    ];

    for (a, b) in test_case {
      assert!(!PaymentMethod::_is_valid_method_combination(&a, &b))
    }
  }

  #[test]
  fn test_is_valid_method_combination() {
    let method_name = PaymentMethodCategoryName::CreditCard;
    let method_kind_name = PaymentMethodKindName::CreditCard(CreditCard::JCB);

    let result = PaymentMethod::is_valid_method_combination(&method_name, &method_kind_name);
    assert!(result.is_ok());
  }
  #[test]
  fn test_is_valid_method_combination_failed() {
    let method_name = PaymentMethodCategoryName::CreditCard;
    let method_kind_name = PaymentMethodKindName::MobilePayment(MobilePayment::PayPal);

    let result = PaymentMethod::is_valid_method_combination(&method_name, &method_kind_name);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PaymentError::InvalidMethodCombination)
  }
}
