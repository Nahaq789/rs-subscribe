use std::fmt;
use thiserror::Error;

#[derive(Debug)]
pub struct PaymentMethodName {
    method: PaymentMethodAggregate,
}

#[derive(Debug, Error)]
enum PaymentMethodError {}

impl PaymentMethodName {
    pub fn new(k: PaymentMethodAggregate) -> Self {
        Self { method: k }
    }
    pub fn get_method_detail_name(&self) -> (String, String) {
        match &self.method {
            PaymentMethodAggregate::CreditCard(card) => {
                (String::from("Credit Card"), card.to_string())
            }
            PaymentMethodAggregate::DigitalMoney(money) => {
                (String::from("Digital Money"), money.to_string())
            }
            PaymentMethodAggregate::DebitCard(card) => {
                (String::from("Debit Card"), card.to_string())
            }
            PaymentMethodAggregate::BankTransfer(transfer) => {
                (String::from("Bank Transfer"), transfer.to_string())
            }
            PaymentMethodAggregate::MobilePayment(payment) => {
                (String::from("Mobile Payment"), payment.to_string())
            }
            PaymentMethodAggregate::DigitalWallet(wallet) => {
                (String::from("Digital Wallet"), wallet.to_string())
            }
            PaymentMethodAggregate::BNPL(bnpl) => (String::from("BNPL"), bnpl.to_string()),
            PaymentMethodAggregate::CarrierBilling(billing) => {
                (String::from("Carrier Billing"), billing.to_string())
            }
        }
    }
}

// impl TryFrom<String> for PaymentMethodName {
//     type Error = PaymentMethodError;
//
//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         match value.as_str() {
//             "aaa" => PaymentMethodName::new()
//         }
//     }
// }

#[derive(Debug)]
pub enum PaymentMethodAggregate {
    CreditCard(CreditCard),
    DigitalMoney(DigitalMoney),
    DebitCard(DebitCard),
    BankTransfer(BankTransfer),
    MobilePayment(MobilePayment),
    DigitalWallet(DigitalWallet),
    BNPL(BNPL),
    CarrierBilling(CarrierBilling),
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
mod tests {
    use super::*;

    #[test]
    fn test_credit_card_methods() {
        let test_cases = vec![
            (CreditCard::Visa, "Credit Card", "Visa"),
            (CreditCard::MasterCard, "Credit Card", "MasterCard"),
            (
                CreditCard::AmericanExpress,
                "Credit Card",
                "American Express",
            ),
            (CreditCard::JCB, "Credit Card", "JCB"),
            (CreditCard::Discover, "Credit Card", "Discover"),
            (CreditCard::DinersClub, "Credit Card", "Diners Club"),
        ];

        for (card, expected_method, expected_detail) in test_cases {
            let result = PaymentMethodName::new(PaymentMethodAggregate::CreditCard(card));
            assert_eq!(
                (expected_method.to_string(), expected_detail.to_string()),
                result.get_method_detail_name()
            );
        }
    }

    #[test]
    fn test_digital_money_methods() {
        let test_cases = vec![
            (DigitalMoney::Suica, "Digital Money", "Suica"),
            (DigitalMoney::Pasmo, "Digital Money", "PASMO"),
            (DigitalMoney::Nanaco, "Digital Money", "nanaco"),
            (DigitalMoney::Waon, "Digital Money", "WAON"),
            (DigitalMoney::RakutenEdy, "Digital Money", "楽天Edy"),
        ];

        for (money, expected_method, expected_detail) in test_cases {
            let result = PaymentMethodName::new(PaymentMethodAggregate::DigitalMoney(money));
            assert_eq!(
                (expected_method.to_string(), expected_detail.to_string()),
                result.get_method_detail_name()
            );
        }
    }

    #[test]
    fn test_mobile_payment_methods() {
        let test_cases = vec![
            (MobilePayment::PayPay, "Mobile Payment", "PayPay"),
            (MobilePayment::LinePay, "Mobile Payment", "LINE Pay"),
            (MobilePayment::MerPay, "Mobile Payment", "メルペイ"),
            (MobilePayment::RakutenPay, "Mobile Payment", "楽天ペイ"),
            (MobilePayment::DBarai, "Mobile Payment", "d払い"),
            (MobilePayment::Venmo, "Mobile Payment", "Venmo"),
            (MobilePayment::CashApp, "Mobile Payment", "Cash App"),
            (MobilePayment::Zelle, "Mobile Payment", "Zelle"),
            (MobilePayment::PayPal, "Mobile Payment", "PayPal"),
        ];

        for (payment, expected_method, expected_detail) in test_cases {
            let result = PaymentMethodName::new(PaymentMethodAggregate::MobilePayment(payment));
            assert_eq!(
                (expected_method.to_string(), expected_detail.to_string()),
                result.get_method_detail_name()
            );
        }
    }

    #[test]
    fn test_digital_wallet_methods() {
        let test_cases = vec![
            (DigitalWallet::ApplePay, "Digital Wallet", "Apple Pay"),
            (DigitalWallet::GooglePay, "Digital Wallet", "Google Pay"),
            (DigitalWallet::AmazonPay, "Digital Wallet", "Amazon Pay"),
        ];

        for (wallet, expected_method, expected_detail) in test_cases {
            let result = PaymentMethodName::new(PaymentMethodAggregate::DigitalWallet(wallet));
            assert_eq!(
                (expected_method.to_string(), expected_detail.to_string()),
                result.get_method_detail_name()
            );
        }
    }

    #[test]
    fn test_bnpl_methods() {
        let test_cases = vec![
            (BNPL::Affirm, "BNPL", "Affirm"),
            (BNPL::Klarna, "BNPL", "Klarna"),
            (BNPL::Afterpay, "BNPL", "Afterpay"),
        ];

        for (bnpl, expected_method, expected_detail) in test_cases {
            let result = PaymentMethodName::new(PaymentMethodAggregate::BNPL(bnpl));
            assert_eq!(
                (expected_method.to_string(), expected_detail.to_string()),
                result.get_method_detail_name()
            );
        }
    }

    #[test]
    fn test_bank_transfer_methods() {
        let test_cases = vec![
            (
                BankTransfer::JapaneseBankTransfer,
                "Bank Transfer",
                "銀行振込",
            ),
            (
                BankTransfer::JapaneseDirectDebit,
                "Bank Transfer",
                "口座振替",
            ),
            (BankTransfer::ACH, "Bank Transfer", "ACH Transfer"),
        ];

        for (transfer, expected_method, expected_detail) in test_cases {
            let result = PaymentMethodName::new(PaymentMethodAggregate::BankTransfer(transfer));
            assert_eq!(
                (expected_method.to_string(), expected_detail.to_string()),
                result.get_method_detail_name()
            );
        }
    }
}
