pub mod payment_method;
pub mod payment_method_id;

use chrono::{DateTime, Utc};
use payment_method::{MethodNameAggregate, PaymentMethodName};
use payment_method_id::PaymentMethodId;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PaymentMethod {
  payment_method_id: PaymentMethodId,
  method_name: String,
  detail_name: String,
  additional_name: Option<String>,
  created_at: Option<DateTime<Utc>>,
  updated_at: Option<DateTime<Utc>>,
}

impl PaymentMethod {
  pub fn new(
    payment_method_id: PaymentMethodId,
    method_name: MethodNameAggregate,
    additional_name: Option<String>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
  ) -> Self {
    let (method_name, detail_name) = PaymentMethodName::new(method_name).get_method_detail_name();
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
