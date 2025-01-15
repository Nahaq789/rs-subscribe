#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscribeDto {
  subscribe_id: String,
  user_id: String,
  name: String,
  payment_method_id: String,
  amount: String,
  payment_cycle: String,
  category_id: String,
  icon_local_path: String,
  notification: bool,
  first_payment_date: chrono::DateTime<chrono::Utc>,
  next_payment_date: chrono::DateTime<chrono::Utc>,
  auto_renewal: bool,
  status: String,
  memo: Option<String>,
}

impl SubscribeDto {
  pub fn new(
    subscribe_id: String,
    user_id: String,
    name: String,
    payment_method_id: String,
    amount: String,
    payment_cycle: String,
    category_id: String,
    icon_local_path: String,
    notification: bool,
    first_payment_date: chrono::DateTime<chrono::Utc>,
    next_payment_date: chrono::DateTime<chrono::Utc>,
    auto_renewal: bool,
    status: String,
    memo: Option<String>,
  ) -> Self {
    Self {
      subscribe_id,
      user_id,
      name,
      payment_method_id,
      amount,
      payment_cycle,
      category_id,
      icon_local_path,
      notification,
      first_payment_date,
      next_payment_date,
      auto_renewal,
      status,
      memo,
    }
  }

  pub fn builder() -> SubscribeDtoBuilder {
    SubscribeDtoBuilder::default()
  }
}

#[derive(Default)]
pub struct SubscribeDtoBuilder {
  subscribe_id: Option<String>,
  user_id: Option<String>,
  name: Option<String>,
  payment_method_id: Option<String>,
  amount: Option<String>,
  payment_cycle: Option<String>,
  category_id: Option<String>,
  icon_local_path: Option<String>,
  notification: Option<bool>,
  first_payment_date: Option<chrono::DateTime<chrono::Utc>>,
  next_payment_date: Option<chrono::DateTime<chrono::Utc>>,
  auto_renewal: Option<bool>,
  status: Option<String>,
  memo: Option<String>,
}
