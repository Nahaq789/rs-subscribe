use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaymentMethodNameError {
  #[error("Invalid payment method category: {0}")]
  InvalidCategoryName(String),

  #[error("Invalid payment method kind: {0}")]
  InvalidKindName(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PaymentMethodCategoryName {
  CreditCard,
  DigitalMoney,
  MobilePayment,
  DigitalWallet,
  BankTransfer,
  BNPL,
  DebitCard,
  CarrierBilling,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PaymentMethodKindName {
  CreditCard(CreditCard),
  DigitalMoney(DigitalMoney),
  MobilePayment(MobilePayment),
  DigitalWallet(DigitalWallet),
  BankTransfer(BankTransfer),
  BNPL(BNPL),
  DebitCard,
  CarrierBilling,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreditCard {
  Visa,
  MasterCard,
  AmericanExpress,
  JCB,
  Discover,
  DinersClub,
}

impl fmt::Display for CreditCard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Visa => write!(f, "Visa"),
      Self::MasterCard => write!(f, "MasterCard"),
      Self::AmericanExpress => write!(f, "American Express"),
      Self::JCB => write!(f, "JCB"),
      Self::Discover => write!(f, "Discover"),
      Self::DinersClub => write!(f, "Diners Club"),
    }
  }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DigitalMoney {
  Suica,
  Pasmo,
  Nanaco,
  Waon,
  RakutenEdy,
}

impl fmt::Display for DigitalMoney {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Suica => write!(f, "Suica"),
      Self::Pasmo => write!(f, "PASMO"),
      Self::Nanaco => write!(f, "nanaco"),
      Self::Waon => write!(f, "WAON"),
      Self::RakutenEdy => write!(f, "楽天Edy"),
    }
  }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MobilePayment {
  PayPay,
  LinePay,
  MerPay,
  RakutenPay,
  DBarai,
  Venmo,
  CashApp,
  Zelle,
  PayPal,
}

impl fmt::Display for MobilePayment {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::PayPay => write!(f, "PayPay"),
      Self::LinePay => write!(f, "LINE Pay"),
      Self::MerPay => write!(f, "メルペイ"),
      Self::RakutenPay => write!(f, "楽天ペイ"),
      Self::DBarai => write!(f, "d払い"),
      Self::Venmo => write!(f, "Venmo"),
      Self::CashApp => write!(f, "Cash App"),
      Self::Zelle => write!(f, "Zelle"),
      Self::PayPal => write!(f, "PayPal"),
    }
  }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DigitalWallet {
  ApplePay,
  GooglePay,
  AmazonPay,
}

impl fmt::Display for DigitalWallet {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::ApplePay => write!(f, "ApplePay"),
      Self::GooglePay => write!(f, "GooglePay"),
      Self::AmazonPay => write!(f, "AmazonPay"),
    }
  }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BankTransfer {
  JapaneseBankTransfer,
  JapaneseDirectDebit,
  ACH,
}

impl fmt::Display for BankTransfer {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::JapaneseBankTransfer => write!(f, "Japanese BankTransfer"),
      Self::JapaneseDirectDebit => write!(f, "Japanese DirectDebit"),
      Self::ACH => write!(f, "ACH"),
    }
  }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BNPL {
  Affirm,
  Klarna,
  Afterpay,
}

impl fmt::Display for BNPL {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Affirm => write!(f, "Affirm"),
      Self::Afterpay => write!(f, "Afterpay"),
      Self::Klarna => write!(f, "Klarna"),
    }
  }
}

impl fmt::Display for PaymentMethodCategoryName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::CreditCard => write!(f, "Credit Card"),
      Self::DigitalMoney => write!(f, "Digital Money"),
      Self::MobilePayment => write!(f, "Mobile Payment"),
      Self::DigitalWallet => write!(f, "Digital Wallet"),
      Self::BankTransfer => write!(f, "Bank Transfer"),
      Self::BNPL => write!(f, "BNPL"),
      Self::DebitCard => write!(f, "Debit Card"),
      Self::CarrierBilling => write!(f, "Carrier Billing"),
    }
  }
}

impl FromStr for PaymentMethodCategoryName {
  type Err = PaymentMethodNameError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Credit Card" => Ok(Self::CreditCard),
      "Digital Money" => Ok(Self::DigitalMoney),
      "Mobile Payment" => Ok(Self::MobilePayment),
      "Digital Wallet" => Ok(Self::DigitalWallet),
      "Bank Transfer" => Ok(Self::BankTransfer),
      "BNPL" => Ok(Self::BNPL),
      "Debit Card" => Ok(Self::DebitCard),
      "Carrier Billing" => Ok(Self::CarrierBilling),

      _ => Err(PaymentMethodNameError::InvalidCategoryName(s.to_string())),
    }
  }
}

impl FromStr for PaymentMethodKindName {
  type Err = PaymentMethodNameError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      // Credit Card
      "Visa" => Ok(Self::CreditCard(CreditCard::Visa)),
      "MasterCard" => Ok(Self::CreditCard(CreditCard::MasterCard)),
      "American Express" => Ok(Self::CreditCard(CreditCard::AmericanExpress)),
      "JCB" => Ok(Self::CreditCard(CreditCard::JCB)),
      "Discover" => Ok(Self::CreditCard(CreditCard::Discover)),
      "Diners Club" => Ok(Self::CreditCard(CreditCard::DinersClub)),

      // Digital Money
      "Suica" => Ok(Self::DigitalMoney(DigitalMoney::Suica)),
      "PASMO" => Ok(Self::DigitalMoney(DigitalMoney::Pasmo)),
      "nanaco" => Ok(Self::DigitalMoney(DigitalMoney::Nanaco)),
      "WAON" => Ok(Self::DigitalMoney(DigitalMoney::Waon)),
      "楽天Edy" => Ok(Self::DigitalMoney(DigitalMoney::RakutenEdy)),

      // Mobile Payment
      "PayPay" => Ok(Self::MobilePayment(MobilePayment::PayPay)),
      "LinePay" => Ok(Self::MobilePayment(MobilePayment::LinePay)),
      "MerPay" => Ok(Self::MobilePayment(MobilePayment::MerPay)),
      "RakutenPay" => Ok(Self::MobilePayment(MobilePayment::RakutenPay)),
      "DBarai" => Ok(Self::MobilePayment(MobilePayment::DBarai)),
      "Venmo" => Ok(Self::MobilePayment(MobilePayment::Venmo)),
      "CashApp" => Ok(Self::MobilePayment(MobilePayment::CashApp)),
      "Zelle" => Ok(Self::MobilePayment(MobilePayment::Zelle)),
      "PayPal" => Ok(Self::MobilePayment(MobilePayment::PayPal)),

      // Digital Wallet
      "ApplePay" => Ok(Self::DigitalWallet(DigitalWallet::ApplePay)),
      "GooglePay" => Ok(Self::DigitalWallet(DigitalWallet::GooglePay)),
      "AmazonPay" => Ok(Self::DigitalWallet(DigitalWallet::AmazonPay)),

      // Bank Transfer
      "JapaneseBankTransfer" => Ok(Self::BankTransfer(BankTransfer::JapaneseBankTransfer)),
      "JapaneseDirectDebit" => Ok(Self::BankTransfer(BankTransfer::JapaneseDirectDebit)),
      "ACH" => Ok(Self::BankTransfer(BankTransfer::ACH)),

      // BNPL
      "Affirm" => Ok(Self::BNPL(BNPL::Affirm)),
      "Klarna" => Ok(Self::BNPL(BNPL::Klarna)),
      "Afterpay" => Ok(Self::BNPL(BNPL::Afterpay)),

      _ => Err(PaymentMethodNameError::InvalidKindName(s.to_string())),
    }
  }
}

impl fmt::Display for PaymentMethodKindName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::CreditCard(cc) => write!(f, "{}", cc),
      Self::DigitalMoney(dm) => write!(f, "{}", dm),
      Self::MobilePayment(mp) => write!(f, "{}", mp),
      Self::DigitalWallet(dw) => write!(f, "{}", dw),
      Self::BankTransfer(bt) => write!(f, "{}", bt),
      Self::BNPL(bnpl) => write!(f, "{}", bnpl),
      Self::DebitCard => write!(f, "デビットカード"),
      Self::CarrierBilling => write!(f, "キャリア決済"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_payment_method_kind_name_from_str() {
    let test_case = vec![
      // Credit Card
      ("Visa", PaymentMethodKindName::CreditCard(CreditCard::Visa)),
      (
        "MasterCard",
        PaymentMethodKindName::CreditCard(CreditCard::MasterCard),
      ),
      (
        "American Express",
        PaymentMethodKindName::CreditCard(CreditCard::AmericanExpress),
      ),
      ("JCB", PaymentMethodKindName::CreditCard(CreditCard::JCB)),
      (
        "Discover",
        PaymentMethodKindName::CreditCard(CreditCard::Discover),
      ),
      (
        "Diners Club",
        PaymentMethodKindName::CreditCard(CreditCard::DinersClub),
      ),
      // Digital Money
      (
        "Suica",
        PaymentMethodKindName::DigitalMoney(DigitalMoney::Suica),
      ),
      (
        "PASMO",
        PaymentMethodKindName::DigitalMoney(DigitalMoney::Pasmo),
      ),
      (
        "nanaco",
        PaymentMethodKindName::DigitalMoney(DigitalMoney::Nanaco),
      ),
      (
        "WAON",
        PaymentMethodKindName::DigitalMoney(DigitalMoney::Waon),
      ),
      (
        "楽天Edy",
        PaymentMethodKindName::DigitalMoney(DigitalMoney::RakutenEdy),
      ),
      // Mobile Payment
      (
        "PayPay",
        PaymentMethodKindName::MobilePayment(MobilePayment::PayPay),
      ),
      (
        "LinePay",
        PaymentMethodKindName::MobilePayment(MobilePayment::LinePay),
      ),
      (
        "MerPay",
        PaymentMethodKindName::MobilePayment(MobilePayment::MerPay),
      ),
      (
        "RakutenPay",
        PaymentMethodKindName::MobilePayment(MobilePayment::RakutenPay),
      ),
      (
        "DBarai",
        PaymentMethodKindName::MobilePayment(MobilePayment::DBarai),
      ),
      (
        "Venmo",
        PaymentMethodKindName::MobilePayment(MobilePayment::Venmo),
      ),
      (
        "CashApp",
        PaymentMethodKindName::MobilePayment(MobilePayment::CashApp),
      ),
      (
        "Zelle",
        PaymentMethodKindName::MobilePayment(MobilePayment::Zelle),
      ),
      (
        "PayPal",
        PaymentMethodKindName::MobilePayment(MobilePayment::PayPal),
      ),
      // Digital Wallet
      (
        "ApplePay",
        PaymentMethodKindName::DigitalWallet(DigitalWallet::ApplePay),
      ),
      (
        "GooglePay",
        PaymentMethodKindName::DigitalWallet(DigitalWallet::GooglePay),
      ),
      (
        "AmazonPay",
        PaymentMethodKindName::DigitalWallet(DigitalWallet::AmazonPay),
      ),
      // Bank Transfer
      (
        "JapaneseBankTransfer",
        PaymentMethodKindName::BankTransfer(BankTransfer::JapaneseBankTransfer),
      ),
      (
        "JapaneseDirectDebit",
        PaymentMethodKindName::BankTransfer(BankTransfer::JapaneseDirectDebit),
      ),
      (
        "ACH",
        PaymentMethodKindName::BankTransfer(BankTransfer::ACH),
      ),
      // BNPL
      ("Affirm", PaymentMethodKindName::BNPL(BNPL::Affirm)),
      ("Klarna", PaymentMethodKindName::BNPL(BNPL::Klarna)),
      ("Afterpay", PaymentMethodKindName::BNPL(BNPL::Afterpay)),
    ];

    for i in test_case {
      let result = PaymentMethodKindName::from_str(i.0).unwrap();
      assert_eq!(result, i.1, "input: {}", i.1)
    }
  }

  #[test]
  fn test_payment_method_kind_name_from_str_error() {
    let error_cases = vec![
      "",
      "Invalid",
      "visa",
      "VISA",
      "mastercard",
      "apple pay",
      "suica",
    ];

    for input in error_cases {
      assert!(
        PaymentMethodKindName::from_str(input).is_err(),
        "input: {}",
        input
      )
    }
  }

  #[test]
  fn test_credit_card_fmt() {
    let test_case = vec![
      ("Visa", CreditCard::Visa.to_string()),
      ("MasterCard", CreditCard::MasterCard.to_string()),
      ("American Express", CreditCard::AmericanExpress.to_string()),
      ("JCB", CreditCard::JCB.to_string()),
      ("Discover", CreditCard::Discover.to_string()),
      ("Diners Club", CreditCard::DinersClub.to_string()),
    ];

    for (expected, actual) in test_case {
      assert_eq!(expected.to_string(), actual)
    }
  }

  #[test]
  fn test_digital_money_fmt() {
    let test_case = vec![
      ("Suica", DigitalMoney::Suica.to_string()),
      ("PASMO", DigitalMoney::Pasmo.to_string()),
      ("nanaco", DigitalMoney::Nanaco.to_string()),
      ("WAON", DigitalMoney::Waon.to_string()),
      ("楽天Edy", DigitalMoney::RakutenEdy.to_string()),
    ];

    for (expected, actual) in test_case {
      assert_eq!(expected.to_string(), actual);
    }
  }

  #[test]
  fn test_mobile_payment_fmt() {
    let test_case = vec![
      ("PayPay", MobilePayment::PayPay.to_string()),
      ("LINE Pay", MobilePayment::LinePay.to_string()),
      ("メルペイ", MobilePayment::MerPay.to_string()),
      ("楽天ペイ", MobilePayment::RakutenPay.to_string()),
      ("d払い", MobilePayment::DBarai.to_string()),
      ("Venmo", MobilePayment::Venmo.to_string()),
      ("Cash App", MobilePayment::CashApp.to_string()),
      ("Zelle", MobilePayment::Zelle.to_string()),
      ("PayPal", MobilePayment::PayPal.to_string()),
    ];

    for (expected, actual) in test_case {
      assert_eq!(expected.to_string(), actual);
    }
  }

  #[test]
  fn test_digital_wallet_fmt() {
    let test_case = vec![
      ("ApplePay", DigitalWallet::ApplePay.to_string()),
      ("GooglePay", DigitalWallet::GooglePay.to_string()),
      ("AmazonPay", DigitalWallet::AmazonPay.to_string()),
    ];

    for (expected, actual) in test_case {
      assert_eq!(expected.to_string(), actual);
    }
  }

  #[test]
  fn test_bank_transfer_fmt() {
    let test_case = vec![
      (
        "Japanese BankTransfer",
        BankTransfer::JapaneseBankTransfer.to_string(),
      ),
      (
        "Japanese DirectDebit",
        BankTransfer::JapaneseDirectDebit.to_string(),
      ),
      ("ACH", BankTransfer::ACH.to_string()),
    ];

    for (expected, actual) in test_case {
      assert_eq!(expected.to_string(), actual);
    }
  }

  #[test]
  fn test_bnpl_fmt() {
    let test_case = vec![
      ("Affirm", BNPL::Affirm.to_string()),
      ("Klarna", BNPL::Klarna.to_string()),
      ("Afterpay", BNPL::Afterpay.to_string()), // Note: This seems to be a bug in the original code
    ];

    for (expected, actual) in test_case {
      assert_eq!(expected.to_string(), actual);
    }
  }

  #[test]
  fn test_payment_method_kind_name_fmt() {
    let test_case = vec![
      (PaymentMethodKindName::CreditCard(CreditCard::Visa), "Visa"),
      (
        PaymentMethodKindName::DigitalMoney(DigitalMoney::Pasmo),
        "PASMO",
      ),
      (
        PaymentMethodKindName::MobilePayment(MobilePayment::CashApp),
        "Cash App",
      ),
      (
        PaymentMethodKindName::DigitalWallet(DigitalWallet::AmazonPay),
        "AmazonPay",
      ),
      (
        PaymentMethodKindName::BankTransfer(BankTransfer::ACH),
        "ACH",
      ),
      (PaymentMethodKindName::BNPL(BNPL::Affirm), "Affirm"),
      (PaymentMethodKindName::DebitCard, "デビットカード"),
      (PaymentMethodKindName::CarrierBilling, "キャリア決済"),
    ];

    for (kind, expected) in test_case {
      assert_eq!(kind.to_string(), expected)
    }
  }

  #[test]
  fn test_payment_method_category_name_from_str() {
    let test_case = create_payment_method_category_test_case();

    for (found, expected) in test_case {
      let result = PaymentMethodCategoryName::from_str(found).unwrap();
      assert_eq!(result, expected)
    }

    let invalid_test_case = PaymentMethodCategoryName::from_str("invalid");
    assert!(matches!(
      invalid_test_case,
      Err(PaymentMethodNameError::InvalidCategoryName(_))
    ))
  }

  #[test]
  fn test_payment_method_category_name_fmt() {
    let test_case = create_payment_method_category_test_case();

    for (expected, found) in test_case {
      let result = found.to_string();
      assert_eq!(expected, &result)
    }
  }

  fn create_payment_method_category_test_case() -> Vec<(&'static str, PaymentMethodCategoryName)> {
    let test_case = vec![
      ("Credit Card", PaymentMethodCategoryName::CreditCard),
      ("Digital Money", PaymentMethodCategoryName::DigitalMoney),
      ("Mobile Payment", PaymentMethodCategoryName::MobilePayment),
      ("Digital Wallet", PaymentMethodCategoryName::DigitalWallet),
      ("Bank Transfer", PaymentMethodCategoryName::BankTransfer),
      ("BNPL", PaymentMethodCategoryName::BNPL),
      ("Debit Card", PaymentMethodCategoryName::DebitCard),
      ("Carrier Billing", PaymentMethodCategoryName::CarrierBilling),
    ];
    test_case
  }
}
