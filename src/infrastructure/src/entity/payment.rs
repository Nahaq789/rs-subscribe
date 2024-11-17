use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentEntity {
  payment_id: String,
  user_id: String,
  method_name: String,
  additional_name: String,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}
