use std::fmt;
use std::fmt::Formatter;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaymentMethodNameError {
    #[error("Invalid payment method: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PaymentMethodName {
    CreditCard(CreditCard),
    DigitalMoney(DigitalMoney),
    MobilePayment(MobilePayment),
    DigitalWallet(DigitalWallet),
    BankTransfer(BankTransfer),
    BNPL(BNPL),
    DebitCard,
    CarrierBilling,
}

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
            Self::Afterpay => write!(f, "Affirm"),
            Self::Klarna => write!(f, "Klarna"),
        }
    }
}

impl PaymentMethodName {
    pub fn from_str(s: &str) -> Result<Self, PaymentMethodNameError> {
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

            _ => Err(PaymentMethodNameError::Invalid(s.to_string())),
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            Self::CreditCard(_) => "Credit Card",
            Self::DigitalMoney(_) => "Digital Money",
            Self::MobilePayment(_) => "Mobile Payment",
            Self::DigitalWallet(_) => "Digital Wallet",
            Self::BankTransfer(_) => "Bank Transfer",
            Self::BNPL(_) => "Buy Now Pay Later",
            Self::DebitCard => "Debit Card",
            Self::CarrierBilling => "Carrier Billing",
        }
    }
}

impl fmt::Display for PaymentMethodName {
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
