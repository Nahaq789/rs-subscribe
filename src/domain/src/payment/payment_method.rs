use std::fmt::{self, Debug, Display};
use thiserror::Error;

pub trait PaymentMethodAggregate: Debug + Display {
    fn clone_box(&self) -> Box<dyn PaymentMethodAggregate>;
}
pub trait DetailMethod: Debug + Display {
    fn clone_box(&self) -> Box<dyn DetailMethod>;
}

#[derive(Debug, Error)]
pub enum PaymentMethodNameError {
    #[error("Method name not found: {0}")]
    MethodNameNotFound(String),
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
    pub fn value(&self) -> &dyn PaymentMethodAggregate {
        &*self.method
    }
}

impl Clone for PaymentMethodName {
    fn clone(&self) -> Self {
        Self {
            method: self.method.clone_box(),
        }
    }
}

impl DetailMethodName {
    pub fn new(d: Box<dyn DetailMethod>) -> Self {
        Self { detail: d }
    }
    pub fn value(&self) -> &dyn DetailMethod {
        &*self.detail
    }
}

impl Clone for DetailMethodName {
    fn clone(&self) -> Self {
        Self {
            detail: self.detail.clone_box(),
        }
    }
}

#[derive(Debug, Clone)]
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
    fn clone_box(&self) -> Box<dyn PaymentMethodAggregate> {
        Box::new(self.clone())
    }
}

impl fmt::Display for MethodNameAggregate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MethodNameAggregate::CreditCard => write!(f, "Credit Card"),
            MethodNameAggregate::DigitalMoney => write!(f, "Digital Money"),
            MethodNameAggregate::DebitCard => write!(f, "Debit Card"),
            MethodNameAggregate::BankTransfer => write!(f, "Bank Transfer"),
            MethodNameAggregate::MobilePayment => write!(f, "Mobile Payment"),
            MethodNameAggregate::DigitalWallet => write!(f, "Digital Wallet"),
            MethodNameAggregate::BNPL => write!(f, "Buy Now Pay Later"),
            MethodNameAggregate::CarrierBilling => write!(f, "Carrier Billing"),
        }
    }
}

impl TryFrom<&str> for MethodNameAggregate {
    type Error = PaymentMethodNameError;

    fn try_from(m: &str) -> Result<Self, Self::Error> {
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

#[derive(Debug, Clone)]
pub enum CreditCard {
    Visa,
    MasterCard,
    AmericanExpress,
    JCB,
    Discover,
    DinersClub,
}

impl DetailMethod for CreditCard {
    fn clone_box(&self) -> Box<dyn DetailMethod> {
        Box::new(self.clone())
    }
}

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
            _ => Err(PaymentMethodNameError::MethodNameNotFound(
                value.to_string(),
            )),
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

#[derive(Debug, Clone)]
pub enum DigitalMoney {
    Suica,
    Pasmo,
    Nanaco,
    Waon,
    RakutenEdy,
}

impl DetailMethod for DigitalMoney {
    fn clone_box(&self) -> Box<dyn DetailMethod> {
        Box::new(self.clone())
    }
}

impl TryFrom<&str> for DigitalMoney {
    type Error = PaymentMethodNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Suica" => Ok(DigitalMoney::Suica),
            "PASMO" => Ok(DigitalMoney::Pasmo),
            "nanaco" => Ok(DigitalMoney::Nanaco),
            "WAON" => Ok(DigitalMoney::Waon),
            "楽天Edy" => Ok(DigitalMoney::RakutenEdy),
            _ => Err(PaymentMethodNameError::MethodNameNotFound(
                value.to_string(),
            )),
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
#[derive(Debug, Clone)]
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

impl DetailMethod for MobilePayment {
    fn clone_box(&self) -> Box<dyn DetailMethod> {
        Box::new(self.clone())
    }
}

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
            _ => Err(PaymentMethodNameError::MethodNameNotFound(
                value.to_string(),
            )),
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
#[derive(Debug, Clone)]
pub enum DigitalWallet {
    ApplePay,
    GooglePay,
    AmazonPay,
}

impl DetailMethod for DigitalWallet {
    fn clone_box(&self) -> Box<dyn DetailMethod> {
        Box::new(self.clone())
    }
}

impl TryFrom<&str> for DigitalWallet {
    type Error = PaymentMethodNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Apple Pay" => Ok(DigitalWallet::ApplePay),
            "Google Pay" => Ok(DigitalWallet::GooglePay),
            "Amazon Pay" => Ok(DigitalWallet::AmazonPay),
            _ => Err(PaymentMethodNameError::MethodNameNotFound(
                value.to_string(),
            )),
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
#[derive(Debug, Clone)]
pub enum BNPL {
    Affirm,
    Klarna,
    Afterpay,
}

impl DetailMethod for BNPL {
    fn clone_box(&self) -> Box<dyn DetailMethod> {
        Box::new(self.clone())
    }
}

impl TryFrom<&str> for BNPL {
    type Error = PaymentMethodNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Affirm" => Ok(BNPL::Affirm),
            "Klarna" => Ok(BNPL::Klarna),
            "Afterpay" => Ok(BNPL::Afterpay),
            _ => Err(PaymentMethodNameError::MethodNameNotFound(
                value.to_string(),
            )),
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
#[derive(Debug, Clone)]
pub enum BankTransfer {
    JapaneseBankTransfer,
    JapaneseDirectDebit,
    ACH,
}

impl DetailMethod for BankTransfer {
    fn clone_box(&self) -> Box<dyn DetailMethod> {
        Box::new(self.clone())
    }
}

impl TryFrom<&str> for BankTransfer {
    type Error = PaymentMethodNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "銀行振込" => Ok(BankTransfer::JapaneseBankTransfer),
            "口座振替" => Ok(BankTransfer::JapaneseDirectDebit),
            "ACH Transfer" => Ok(BankTransfer::ACH),
            _ => Err(PaymentMethodNameError::MethodNameNotFound(
                value.to_string(),
            )),
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
#[derive(Debug, Clone)]
pub enum DebitCard {}
impl DetailMethod for DebitCard {
    fn clone_box(&self) -> Box<dyn DetailMethod> {
        Box::new(self.clone())
    }
}

impl fmt::Display for DebitCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "デビットカード")
    }
}

// キャリア決済
#[derive(Debug, Clone)]
pub enum CarrierBilling {}
impl DetailMethod for CarrierBilling {
    fn clone_box(&self) -> Box<dyn DetailMethod> {
        Box::new(self.clone())
    }
}

impl fmt::Display for CarrierBilling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "キャリア決済")
    }
}

#[cfg(test)]
mod tests {}
