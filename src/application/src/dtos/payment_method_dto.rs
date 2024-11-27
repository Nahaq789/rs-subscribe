use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDTO {
  pub payment_method_id: String,
  pub user_id: String,
  pub method_name: String,
  pub method_kind_name: String,
  pub additional_name: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: Option<DateTime<Utc>>,
}