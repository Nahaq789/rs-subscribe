mod payment_method_id;

use chrono::{DateTime, Utc};
use payment_method_id::PaymentMethodId;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PaymentMethod {
    payment_method_id: PaymentMethodId,
    method_name: String,
    detail_name: String,
    additional_name: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
