use crate::error::ApplicationError;

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

impl SubscribeDtoBuilder {
  pub fn subscribe_id(mut self, subscribe_id: String) -> Self {
    self.subscribe_id = Some(subscribe_id);
    self
  }

  pub fn user_id(mut self, user_id: String) -> Self {
    self.user_id = Some(user_id);
    self
  }

  pub fn name(mut self, name: String) -> Self {
    self.name = Some(name);
    self
  }

  pub fn payment_method_id(mut self, payment_method_id: String) -> Self {
    self.payment_method_id = Some(payment_method_id);
    self
  }

  pub fn amount(mut self, amount: String) -> Self {
    self.amount = Some(amount);
    self
  }

  pub fn payment_cycle(mut self, payment_cycle: String) -> Self {
    self.payment_cycle = Some(payment_cycle);
    self
  }

  pub fn category_id(mut self, category_id: String) -> Self {
    self.category_id = Some(category_id);
    self
  }

  pub fn icon_local_path(mut self, icon_local_path: String) -> Self {
    self.icon_local_path = Some(icon_local_path);
    self
  }

  pub fn notification(mut self, notification: bool) -> Self {
    self.notification = Some(notification);
    self
  }

  pub fn first_payment_date(mut self, first_payment_date: chrono::DateTime<chrono::Utc>) -> Self {
    self.first_payment_date = Some(first_payment_date);
    self
  }

  pub fn next_payment_date(mut self, next_payment_date: chrono::DateTime<chrono::Utc>) -> Self {
    self.next_payment_date = Some(next_payment_date);
    self
  }

  pub fn auto_renewal(mut self, auto_renewal: bool) -> Self {
    self.auto_renewal = Some(auto_renewal);
    self
  }

  pub fn status(mut self, status: String) -> Self {
    self.status = Some(status);
    self
  }

  pub fn memo(mut self, memo: Option<String>) -> Self {
    self.memo = memo;
    self
  }

  pub fn build(self) -> Result<SubscribeDto, String> {
    Ok(SubscribeDto {
      subscribe_id: self.subscribe_id.ok_or("subscribe_id is required")?,
      user_id: self.user_id.ok_or("user_id is required")?,
      name: self.name.ok_or("name is required")?,
      payment_method_id: self
        .payment_method_id
        .ok_or("payment_method_id is required")?,
      amount: self.amount.ok_or("amount is required")?,
      payment_cycle: self.payment_cycle.ok_or("payment_cycle is required")?,
      category_id: self.category_id.ok_or("category_id is required")?,
      icon_local_path: self.icon_local_path.ok_or("icon_local_path is required")?,
      notification: self.notification.unwrap_or(false),
      first_payment_date: self
        .first_payment_date
        .ok_or("first_payment_date is required")?,
      next_payment_date: self
        .next_payment_date
        .ok_or("next_payment_date is required")?,
      auto_renewal: self.auto_renewal.unwrap_or(true),
      status: self.status.ok_or("status is required")?,
      memo: self.memo,
    })
  }
}

impl crate::dtos::DTO<SubscribeDto, domain::subscribe::Subscribe, ApplicationError>
  for SubscribeDto
{
  fn map_to_domain_model(
    v: SubscribeDto,
  ) -> Result<domain::subscribe::Subscribe, ApplicationError> {
    todo!()
  }

  fn map_to_dto(v: &domain::subscribe::Subscribe) -> SubscribeDto {
    todo!()
  }
}
