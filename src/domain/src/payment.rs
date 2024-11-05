pub mod payment_method;
pub mod payment_method_id;

use chrono::{DateTime, Utc};
use payment_method::{DetailMethod, PaymentMethodAggregate};
use payment_method_id::PaymentMethodId;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PaymentMethod<D, M>
where
    D: DetailMethod,
    M: PaymentMethodAggregate,
{
    payment_method_id: PaymentMethodId,
    method_name: M,
    detail_name: D,
    additional_name: Option<String>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl<D: DetailMethod, M: PaymentMethodAggregate> PaymentMethod<D, M> {
    pub fn new(
        payment_method_id: PaymentMethodId,
        method_name: M,
        detail_name: D,
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
