pub mod payment_method;
pub mod payment_method_id;

use chrono::{DateTime, Utc};
use payment_method::{DetailMethodName, PaymentMethodName};
use payment_method_id::PaymentMethodId;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PaymentMethod {
    payment_method_id: PaymentMethodId,
    method_name: PaymentMethodName,
    detail_name: DetailMethodName,
    additional_name: Option<String>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl PaymentMethod {
    pub fn new(
        payment_method_id: PaymentMethodId,
        method_name: PaymentMethodName,
        detail_name: DetailMethodName,
        additional_name: Option<String>,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            payment_method_id,
            method_name,
            detail_name,
            additional_name,
            created_at,
            updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use payment_method::{CreditCard, MethodNameAggregate};

    #[test]
    fn test_payment_method_new() {
        let payment_method_id = PaymentMethodId::new();
        let method_name = PaymentMethodName::new(Box::new(MethodNameAggregate::CreditCard));
        let detail_name = DetailMethodName::new(Box::new(CreditCard::Visa));
        let additional_name = Some("hoge".to_string());
        let created_at = Some(Utc::now());
        let updated_at = Some(Utc::now());

        let result = PaymentMethod::new(
            payment_method_id.clone(),
            method_name.clone(),
            detail_name.clone(),
            additional_name.clone(),
            created_at,
            updated_at,
        );

        assert_eq!(payment_method_id, result.payment_method_id);
        assert!(PaymentMethodName::eq(&method_name, &result.method_name));
        assert!(DetailMethodName::eq(&detail_name, &result.detail_name));
        assert!(additional_name.is_some());
        assert_eq!(additional_name, result.additional_name);
        assert!(created_at.is_some());
        assert_eq!(created_at, result.created_at);
        assert!(updated_at.is_some());
        assert_eq!(updated_at, result.updated_at);
    }
}
