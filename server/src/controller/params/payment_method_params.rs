use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaymentParams {
  pub user_id: String,
  pub payment_id: String,
}
