use std::fmt::{self, Debug, Display};
use thiserror::Error;

pub trait PaymentMethodAggregate: Debug + Display {
    fn clone_box(&self) -> Box<dyn PaymentMethodAggregate>;
}
pub trait DetailMethodAggregate: Debug + Display {
    fn clone_box(&self) -> Box<dyn DetailMethodAggregate>;
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
    detail: Box<dyn DetailMethodAggregate>,
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

impl PartialEq for PaymentMethodName {
    fn eq(&self, other: &Self) -> bool {
        self.method.to_string() == other.method.to_string()
    }
}

impl Eq for PaymentMethodName {}

impl DetailMethodName {
    pub fn new(d: Box<dyn DetailMethodAggregate>) -> Self {
        Self { detail: d }
    }
    pub fn value(&self) -> &dyn DetailMethodAggregate {
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

impl PartialEq for DetailMethodName {
    fn eq(&self, other: &Self) -> bool {
        self.detail.to_string() == other.detail.to_string()
    }
}

impl Eq for DetailMethodName {}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CreditCard {
    Visa,
    MasterCard,
    AmericanExpress,
    JCB,
    Discover,
    DinersClub,
}

impl DetailMethodAggregate for CreditCard {
    fn clone_box(&self) -> Box<dyn DetailMethodAggregate> {
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DigitalMoney {
    Suica,
    Pasmo,
    Nanaco,
    Waon,
    RakutenEdy,
}

impl DetailMethodAggregate for DigitalMoney {
    fn clone_box(&self) -> Box<dyn DetailMethodAggregate> {
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
#[derive(Debug, Clone, Eq, PartialEq)]
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

impl DetailMethodAggregate for MobilePayment {
    fn clone_box(&self) -> Box<dyn DetailMethodAggregate> {
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DigitalWallet {
    ApplePay,
    GooglePay,
    AmazonPay,
}

impl DetailMethodAggregate for DigitalWallet {
    fn clone_box(&self) -> Box<dyn DetailMethodAggregate> {
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BNPL {
    Affirm,
    Klarna,
    Afterpay,
}

impl DetailMethodAggregate for BNPL {
    fn clone_box(&self) -> Box<dyn DetailMethodAggregate> {
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BankTransfer {
    JapaneseBankTransfer,
    JapaneseDirectDebit,
    ACH,
}

impl DetailMethodAggregate for BankTransfer {
    fn clone_box(&self) -> Box<dyn DetailMethodAggregate> {
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DebitCard {
    None
}
impl DetailMethodAggregate for DebitCard {
    fn clone_box(&self) -> Box<dyn DetailMethodAggregate> {
        Box::new(self.clone())
    }
}

impl fmt::Display for DebitCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "デビットカード")
    }
}

// キャリア決済
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CarrierBilling {
    None
}
impl DetailMethodAggregate for CarrierBilling {
    fn clone_box(&self) -> Box<dyn DetailMethodAggregate> {
        Box::new(self.clone())
    }
}

impl fmt::Display for CarrierBilling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "キャリア決済")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod method_name_aggregate_tests {
        use super::*;

        #[test]
        fn test_all_variants_conversion() {
            let test_cases = vec![
                ("Credit Card", MethodNameAggregate::CreditCard),
                ("Digital Money", MethodNameAggregate::DigitalMoney),
                ("Debit Card", MethodNameAggregate::DebitCard),
                ("Bank Transfer", MethodNameAggregate::BankTransfer),
                ("Mobile Payment", MethodNameAggregate::MobilePayment),
                ("Digital Wallet", MethodNameAggregate::DigitalWallet),
                ("Buy Now Pay Later", MethodNameAggregate::BNPL),
                ("Carrier Billing", MethodNameAggregate::CarrierBilling),
            ];

            for (input, expected) in test_cases {
                let method = MethodNameAggregate::try_from(input).unwrap();
                assert_eq!(method, expected);
                assert_eq!(method.to_string(), input);
            }
        }

        #[test]
        fn test_invalid_conversion() {
            let result = MethodNameAggregate::try_from("Invalid Method");
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err().to_string(),
                "Method name not found: Invalid Method"
            );
        }

        #[test]
        fn test_clone_box() {
            let original = Box::new(MethodNameAggregate::CreditCard);
            let cloned = original.clone_box();
            assert_eq!(original.to_string(), cloned.to_string());
        }
    }

    mod payment_method_name_tests {
        use super::*;

        #[test]
        fn test_new_and_value() {
            let method = PaymentMethodName::new(Box::new(MethodNameAggregate::CreditCard));
            assert_eq!(method.value().to_string(), "Credit Card");
        }

        #[test]
        fn test_clone() {
            let original = PaymentMethodName::new(Box::new(MethodNameAggregate::CreditCard));
            let cloned = original.clone();
            assert_eq!(original, cloned);
        }

        #[test]
        fn test_equality() {
            let method1 = PaymentMethodName::new(Box::new(MethodNameAggregate::CreditCard));
            let method2 = PaymentMethodName::new(Box::new(MethodNameAggregate::CreditCard));
            let method3 = PaymentMethodName::new(Box::new(MethodNameAggregate::DigitalMoney));

            assert_eq!(method1, method2);
            assert_ne!(method1, method3);
        }
    }

    mod detail_method_name_tests {
        use super::*;

        #[test]
        fn test_new_and_value() {
            let detail = DetailMethodName::new(Box::new(CreditCard::Visa));
            assert_eq!(detail.value().to_string(), "Visa");
        }

        #[test]
        fn test_clone() {
            let original = DetailMethodName::new(Box::new(CreditCard::Visa));
            let cloned = original.clone();
            assert_eq!(original.value().to_string(), cloned.value().to_string());
        }

        #[test]
        fn test_equality() {
            let detail1 = DetailMethodName::new(Box::new(CreditCard::Visa));
            let detail2 = DetailMethodName::new(Box::new(CreditCard::Visa));
            let detail3 = DetailMethodName::new(Box::new(CreditCard::MasterCard));

            assert_eq!(detail1, detail2);
            assert_ne!(detail1, detail3);
        }
    }

    mod credit_card_tests {
        use super::*;

        #[test]
        fn test_all_variants() {
            let test_cases = vec![
                ("Visa", CreditCard::Visa),
                ("MasterCard", CreditCard::MasterCard),
                ("American Express", CreditCard::AmericanExpress),
                ("JCB", CreditCard::JCB),
                ("Discover", CreditCard::Discover),
                ("Diners Club", CreditCard::DinersClub),
            ];

            for (input, expected) in test_cases {
                let card = CreditCard::try_from(input).unwrap();
                assert_eq!(card, expected);
                assert_eq!(card.to_string(), input);
            }
        }

        #[test]
        fn test_invalid_conversion() {
            let result = CreditCard::try_from("Invalid Card");
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err().to_string(),
                "Method name not found: Invalid Card"
            );
        }

        #[test]
        fn test_clone_box() {
            let original = Box::new(CreditCard::Visa);
            let cloned = original.clone_box();
            assert_eq!(original.to_string(), cloned.to_string());
        }
    }

    mod digital_money_tests {
        use super::*;

        #[test]
        fn test_all_variants() {
            let test_cases = vec![
                ("Suica", DigitalMoney::Suica),
                ("PASMO", DigitalMoney::Pasmo),
                ("nanaco", DigitalMoney::Nanaco),
                ("WAON", DigitalMoney::Waon),
                ("楽天Edy", DigitalMoney::RakutenEdy),
            ];

            for (input, expected) in test_cases {
                let money = DigitalMoney::try_from(input).unwrap();
                assert_eq!(money, expected);
                assert_eq!(money.to_string(), input);
            }
        }

        #[test]
        fn test_invalid_conversion() {
            let result = DigitalMoney::try_from("Invalid Money");
            assert!(result.is_err());
        }

        #[test]
        fn test_clone_box() {
            let original = Box::new(DigitalMoney::Suica);
            let cloned = original.clone_box();
            assert_eq!(original.to_string(), cloned.to_string());
        }
    }

    mod mobile_payment_tests {
        use super::*;

        #[test]
        fn test_all_variants() {
            let test_cases = vec![
                ("PayPay", MobilePayment::PayPay),
                ("LINE Pay", MobilePayment::LinePay),
                ("メルペイ", MobilePayment::MerPay),
                ("楽天ペイ", MobilePayment::RakutenPay),
                ("d払い", MobilePayment::DBarai),
                ("Venmo", MobilePayment::Venmo),
                ("Cash App", MobilePayment::CashApp),
                ("Zelle", MobilePayment::Zelle),
                ("PayPal", MobilePayment::PayPal),
            ];

            for (input, expected) in test_cases {
                let payment = MobilePayment::try_from(input).unwrap();
                assert_eq!(payment, expected);
                assert_eq!(payment.to_string(), input);
            }
        }

        #[test]
        fn test_invalid_conversion() {
            let result = MobilePayment::try_from("Invalid Payment");
            assert!(result.is_err());
        }

        #[test]
        fn test_clone_box() {
            let original = Box::new(MobilePayment::PayPay);
            let cloned = original.clone_box();
            assert_eq!(original.to_string(), cloned.to_string());
        }
    }

    mod digital_wallet_tests {
        use super::*;

        #[test]
        fn test_all_variants() {
            let test_cases = vec![
                ("Apple Pay", DigitalWallet::ApplePay),
                ("Google Pay", DigitalWallet::GooglePay),
                ("Amazon Pay", DigitalWallet::AmazonPay),
            ];

            for (input, expected) in test_cases {
                let wallet = DigitalWallet::try_from(input).unwrap();
                assert_eq!(wallet, expected);
                assert_eq!(wallet.to_string(), input);
            }
        }

        #[test]
        fn test_invalid_conversion() {
            let result = DigitalWallet::try_from("Invalid Wallet");
            assert!(result.is_err());
        }

        #[test]
        fn test_clone_box() {
            let original = Box::new(DigitalWallet::ApplePay);
            let cloned = original.clone_box();
            assert_eq!(original.to_string(), cloned.to_string());
        }
    }

    mod bnpl_tests {
        use super::*;

        #[test]
        fn test_all_variants() {
            let test_cases = vec![
                ("Affirm", BNPL::Affirm),
                ("Klarna", BNPL::Klarna),
                ("Afterpay", BNPL::Afterpay),
            ];

            for (input, expected) in test_cases {
                let bnpl = BNPL::try_from(input).unwrap();
                assert_eq!(bnpl, expected);
                assert_eq!(bnpl.to_string(), input);
            }
        }

        #[test]
        fn test_invalid_conversion() {
            let result = BNPL::try_from("Invalid BNPL");
            assert!(result.is_err());
        }

        #[test]
        fn test_clone_box() {
            let original = Box::new(BNPL::Affirm);
            let cloned = original.clone_box();
            assert_eq!(original.to_string(), cloned.to_string());
        }
    }

    mod bank_transfer_tests {
        use super::*;

        #[test]
        fn test_all_variants() {
            let test_cases = vec![
                ("銀行振込", BankTransfer::JapaneseBankTransfer),
                ("口座振替", BankTransfer::JapaneseDirectDebit),
                ("ACH Transfer", BankTransfer::ACH),
            ];

            for (input, expected) in test_cases {
                let transfer = BankTransfer::try_from(input).unwrap();
                assert_eq!(transfer, expected);
                assert_eq!(transfer.to_string(), input);
            }
        }

        #[test]
        fn test_invalid_conversion() {
            let result = BankTransfer::try_from("Invalid Transfer");
            assert!(result.is_err());
        }

        #[test]
        fn test_clone_box() {
            let original = Box::new(BankTransfer::JapaneseBankTransfer);
            let cloned = original.clone_box();
            assert_eq!(original.to_string(), cloned.to_string());
        }
    }

    mod debit_card_tests {
        use super::*;

        #[test]
        fn test_display() {
            let debit_card = DebitCard::None;
            assert_eq!(debit_card.to_string(), "デビットカード");
        }

        #[test]
        fn test_clone_box() {
            let original = Box::new(DebitCard::None);
            let cloned = original.clone_box();
            assert_eq!(original.to_string(), cloned.to_string());
        }
    }

    mod carrier_billing_tests {
        use super::*;

        #[test]
        fn test_display() {
            let carrier_billing = CarrierBilling::None;
            assert_eq!(carrier_billing.to_string(), "キャリア決済");
        }

        #[test]
        fn test_clone_box() {
            let original = Box::new(CarrierBilling::None);
            let cloned = original.clone_box();
            assert_eq!(original.to_string(), cloned.to_string());
        }
    }

    mod error_tests {
        use super::*;

        #[test]
        fn test_error_display() {
            let error = PaymentMethodNameError::MethodNameNotFound("Test".to_string());
            assert_eq!(error.to_string(), "Method name not found: Test");
        }
    }
}
