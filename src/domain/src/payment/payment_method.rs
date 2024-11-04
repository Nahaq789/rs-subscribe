use std::fmt;
use thiserror::Error;

pub trait PaymentMethodAggregate {
  fn get_name(&self) -> Result<String, PaymentMethodNameError>;
  fn try_from(m: &str) -> Result<MethodNameAggregate, PaymentMethodNameError>;
}

pub trait DetailMethod {}

#[derive(Debug, Error)]
enum PaymentMethodNameError {
  #[error("Method name not found: {0}")]
  MethodNameNotFound(String)
}

#[derive(Debug)]
pub struct PaymentMethodName {
  method: Box<dyn PaymentMethodAggregate>,

}

#[derive(Debug)]
pub struct DetailMethodName {
  detail: Box<dyn DetailMethod>,
}

impl PaymentMethodName {
  pub fn new(m: Box<dyn PaymentMethodAggregate>) -> Self {
    Self { method: m }
  }
}

impl DetailMethodName {
  pub fn new(d: Box<dyn DetailMethod>) -> Self {
    Self { detail: d }
  }
}

#[derive(Debug)]
pub enum MethodNameAggregate {
  CreditCard,
  DigitalMoney,
  DebitCard,
  BankTransfer,
  MobilePayment,
  DigitalWallet,
  BNPL,
  CarrierBilling,
}

impl PaymentMethodAggregate for MethodNameAggregate {
  fn get_name(&self) -> Result<String, PaymentMethodNameError> {
    match self {
      MethodNameAggregate::CreditCard => Ok(String::from("Credit Card")),
      MethodNameAggregate::DigitalMoney => Ok(String::from("Digital Money")),
      MethodNameAggregate::DebitCard => Ok(String::from("Debit Card")),
      MethodNameAggregate::BankTransfer => Ok(String::from("Bank Transfer")),
      MethodNameAggregate::MobilePayment => Ok(String::from("Mobile Payment")),
      MethodNameAggregate::DigitalWallet => Ok(String::from("Digital Wallet")),
      MethodNameAggregate::BNPL => Ok(String::from("Buy Now Pay Later")),
      MethodNameAggregate::CarrierBilling => Ok(String::from("Carrier Billing")),
    }
  }

  fn try_from(m: &str) -> Result<MethodNameAggregate, PaymentMethodNameError> {
    match m {
      "Credit Card" => Ok(MethodNameAggregate::CreditCard),
      "Digital Money" => Ok(MethodNameAggregate::DigitalMoney),
      "Debit Card" => Ok(MethodNameAggregate::DebitCard),
      "Bank Transfer" => Ok(MethodNameAggregate::BankTransfer),
      "Mobile Payment" => Ok(MethodNameAggregate::MobilePayment),
      "Digital Wallet" => Ok(MethodNameAggregate::DigitalWallet),
      "Buy Now Pay Later" => Ok(MethodNameAggregate::BNPL),
      "Carrier Billing" => Ok(MethodNameAggregate::CarrierBilling),
      _ => Err(PaymentMethodNameError::MethodNameNotFound(m.to_string())),
    }
  }
}

#[derive(Debug)]
pub enum CreditCard {
  Visa,
  MasterCard,
  AmericanExpress,
  JCB,
  Discover,
  DinersClub,
}

impl DetailMethod for CreditCard {}

impl TryFrom<&str> for CreditCard {
  type Error = PaymentMethodNameError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "Visa" => Ok(CreditCard::Visa),
      "MasterCard" => Ok(CreditCard::MasterCard),
      "American Express" => Ok(CreditCard::AmericanExpress),
      "JCB" => Ok(CreditCard::JCB),
      "Discover" => Ok(CreditCard::Discover),
      "Diners Club" => Ok(CreditCard::DinersClub),
      _ => Err(PaymentMethodNameError::MethodNameNotFound(value.to_string()))
    }
  }
}

impl fmt::Display for CreditCard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CreditCard::Visa => write!(f, "Visa"),
      CreditCard::MasterCard => write!(f, "MasterCard"),
      CreditCard::AmericanExpress => write!(f, "American Express"),
      CreditCard::JCB => write!(f, "JCB"),
      CreditCard::Discover => write!(f, "Discover"),
      CreditCard::DinersClub => write!(f, "Diners Club"),
    }
  }
}

#[derive(Debug)]
pub enum DigitalMoney {
  Suica,
  Pasmo,
  Nanaco,
  Waon,
  RakutenEdy,
}

impl DetailMethod for DigitalMoney {}

impl TryFrom<&str> for DigitalMoney {
  type Error = PaymentMethodNameError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "Suica" => Ok(DigitalMoney::Suica),
      "PASMO" => Ok(DigitalMoney::Pasmo),
      "nanaco" => Ok(DigitalMoney::Nanaco),
      "WAON" => Ok(DigitalMoney::Waon),
      "楽天Edy" => Ok(DigitalMoney::RakutenEdy),
      _ => Err(PaymentMethodNameError::MethodNameNotFound(value.to_string()))
    }
  }
}

impl fmt::Display for DigitalMoney {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      DigitalMoney::Suica => write!(f, "Suica"),
      DigitalMoney::Pasmo => write!(f, "PASMO"),
      DigitalMoney::Nanaco => write!(f, "nanaco"),
      DigitalMoney::Waon => write!(f, "WAON"),
      DigitalMoney::RakutenEdy => write!(f, "楽天Edy"),
    }
  }
}

// モバイル決済
#[derive(Debug)]
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

impl DetailMethod for MobilePayment {}

impl TryFrom<&str> for MobilePayment {
  type Error = PaymentMethodNameError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "PayPay" => Ok(MobilePayment::PayPay),
      "LINE Pay" => Ok(MobilePayment::LinePay),
      "メルペイ" => Ok(MobilePayment::MerPay),
      "楽天ペイ" => Ok(MobilePayment::RakutenPay),
      "d払い" => Ok(MobilePayment::DBarai),
      "Venmo" => Ok(MobilePayment::Venmo),
      "Cash App" => Ok(MobilePayment::CashApp),
      "Zelle" => Ok(MobilePayment::Zelle),
      "PayPal" => Ok(MobilePayment::PayPal),
      _ => Err(PaymentMethodNameError::MethodNameNotFound(value.to_string()))
    }
  }
}

impl fmt::Display for MobilePayment {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MobilePayment::PayPay => write!(f, "PayPay"),
      MobilePayment::LinePay => write!(f, "LINE Pay"),
      MobilePayment::MerPay => write!(f, "メルペイ"),
      MobilePayment::RakutenPay => write!(f, "楽天ペイ"),
      MobilePayment::DBarai => write!(f, "d払い"),
      MobilePayment::Venmo => write!(f, "Venmo"),
      MobilePayment::CashApp => write!(f, "Cash App"),
      MobilePayment::Zelle => write!(f, "Zelle"),
      MobilePayment::PayPal => write!(f, "PayPal"),
    }
  }
}

// デジタルウォレット
#[derive(Debug)]
pub enum DigitalWallet {
  ApplePay,
  GooglePay,
  AmazonPay,
}

impl DetailMethod for DigitalWallet {}

impl TryFrom<&str> for DigitalWallet {
  type Error = PaymentMethodNameError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "Apple Pay" => Ok(DigitalWallet::ApplePay),
      "Google Pay" => Ok(DigitalWallet::GooglePay),
      "Amazon Pay" => Ok(DigitalWallet::AmazonPay),
      _ => Err(PaymentMethodNameError::MethodNameNotFound(value.to_string()))
    }
  }
}

impl fmt::Display for DigitalWallet {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      DigitalWallet::ApplePay => write!(f, "Apple Pay"),
      DigitalWallet::GooglePay => write!(f, "Google Pay"),
      DigitalWallet::AmazonPay => write!(f, "Amazon Pay"),
    }
  }
}

// 後払い決済
#[derive(Debug)]
pub enum BNPL {
  Affirm,
  Klarna,
  Afterpay,
}

impl DetailMethod for BNPL {}

impl TryFrom<&str> for BNPL {
  type Error = PaymentMethodNameError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "Affirm" => Ok(BNPL::Affirm),
      "Klarna" => Ok(BNPL::Klarna),
      "Afterpay" => Ok(BNPL::Afterpay),
      _ => Err(PaymentMethodNameError::MethodNameNotFound(value.to_string()))
    }
  }
}

impl fmt::Display for BNPL {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      BNPL::Affirm => write!(f, "Affirm"),
      BNPL::Klarna => write!(f, "Klarna"),
      BNPL::Afterpay => write!(f, "Afterpay"),
    }
  }
}

// 銀行振込
#[derive(Debug)]
pub enum BankTransfer {
  JapaneseBankTransfer,
  JapaneseDirectDebit,
  ACH,
}

impl DetailMethod for BankTransfer {}

impl TryFrom<&str> for BankTransfer {
  type Error = PaymentMethodNameError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "銀行振込" => Ok(BankTransfer::JapaneseBankTransfer),
      "口座振替" => Ok(BankTransfer::JapaneseDirectDebit),
      "ACH Transfer" => Ok(BankTransfer::ACH),
      _ => Err(PaymentMethodNameError::MethodNameNotFound(value.to_string()))
    }
  }
}

impl fmt::Display for BankTransfer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      BankTransfer::JapaneseBankTransfer => write!(f, "銀行振込"),
      BankTransfer::JapaneseDirectDebit => write!(f, "口座振替"),
      BankTransfer::ACH => write!(f, "ACH Transfer"),
    }
  }
}

// デビットカード
#[derive(Debug)]
pub enum DebitCard {}
impl DetailMethod for DebitCard {}

impl fmt::Display for DebitCard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "デビットカード")
  }
}

// キャリア決済
#[derive(Debug)]
pub enum CarrierBilling {}

impl fmt::Display for CarrierBilling {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "キャリア決済")
  }
}

#[cfg(test)]
mod tests {}
