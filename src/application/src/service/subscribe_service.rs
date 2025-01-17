use std::str::FromStr;

use crate::dtos::{self, DTO};
use anyhow::Ok;

pub struct SubscribeServiceImpl<T: domain::repository::subscribe_repository::SubscribeRepository> {
  repository: T,
}

impl<T: domain::repository::subscribe_repository::SubscribeRepository> SubscribeServiceImpl<T> {
  pub fn new(repository: T) -> SubscribeServiceImpl<T> {
    Self { repository }
  }
}

impl<T: domain::repository::subscribe_repository::SubscribeRepository> crate::service::SubscribeService
  for SubscribeServiceImpl<T>
{
  fn create_subscribe(
    &self,
    subscribe: crate::dtos::subscribe_dto::SubscribeDto,
  ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + '_>> {
    let result = Box::pin(async move {
      let subscribe = crate::dtos::subscribe_dto::SubscribeDto::map_to_domain_model(subscribe)?;
      let result = self.repository.create(&subscribe).await?;
      Ok(result)
    });
    result
  }

  fn find_subscribe_all<'a>(
    &'a self,
    user_id: &'a str,
  ) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = anyhow::Result<Vec<crate::dtos::subscribe_dto::SubscribeDto>>> + Send + '_>,
  > {
    let result = Box::pin(async move {
      let user_id = domain::user::user_id::UserId::from_str(user_id)?;
      let v = self.repository.find_all(&user_id).await?;
      let result = v.into_iter().map(|item| crate::dtos::subscribe_dto::SubscribeDto::map_to_dto(&item)).collect();

      Ok(result)
    });
    result
  }

  fn find_subscribe_by_id<'a>(
    &'a self,
    user_id: &'a str,
    subscribe_id: &'a str,
  ) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = anyhow::Result<crate::dtos::subscribe_dto::SubscribeDto>> + Send + '_>,
  > {
    let result = Box::pin(async move {
      let user_id = domain::user::user_id::UserId::from_str(user_id)?;
      let subscribe_id = domain::subscribe::subscribe_id::SubscribeId::from_str(subscribe_id)?;
      let v = self.repository.find_by_id(&subscribe_id, &user_id).await?;
      let result = crate::dtos::subscribe_dto::SubscribeDto::map_to_dto(&v);

      Ok(result)
    });
    result
  }

  fn update_subscribe(
    &self,
    subscribe: crate::dtos::subscribe_dto::SubscribeDto,
  ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + '_>> {
    let result = Box::pin(async move {
      let subscribe = dtos::subscribe_dto::SubscribeDto::map_to_domain_model(subscribe)?;
      let result = self.repository.update(&subscribe).await?;
      Ok(result)
    });
    result
  }

  fn delete_subscribe<'a>(
    &'a self,
    user_id: &'a str,
    subscribe_id: &'a str,
  ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + '_>> {
    let result = Box::pin(async move {
      let user_id = domain::user::user_id::UserId::from_str(user_id)?;
      let subscribe_id = domain::subscribe::subscribe_id::SubscribeId::from_str(subscribe_id)?;
      let result = self.repository.delete(&subscribe_id, &user_id).await?;

      Ok(result)
    });
    result
  }
}

#[cfg(test)]
mod tests {
  use crate::dtos::subscribe_dto::SubscribeDto;
  use chrono::Utc;
  use domain::category::category_id::CategoryId;
  use domain::payment::payment_method_id::PaymentMethodId;
  use domain::repository::subscribe_repository::SubscribeRepository;
  use domain::subscribe::{subscribe_error::SubscribeError, subscribe_id::SubscribeId, Subscribe, subscribe_name::SubscribeName, subscribe_status::SubscribeStatus};
  use domain::payment_cycle::PaymentCycle;
  use domain::value_object::amount::Amount;
  use domain::user::user_id::UserId;
  use mockall::mock;
  use rust_decimal::Decimal;

  mock! {
    SubscribeRepository {}
    #[async_trait::async_trait]
    impl SubscribeRepository for SubscribeRepository {
      async fn create(&self, subscribe: &Subscribe) -> Result<(), SubscribeError>;
      async fn find_all(&self, user_id: &UserId) -> Result<Vec<Subscribe>, SubscribeError>;
      async fn find_by_id(&self, subscribe_id: &SubscribeId, user_id: &UserId) -> Result<Subscribe, SubscribeError>;
      async fn update(&self, subscribe: &Subscribe) -> Result<(), SubscribeError>;
      async fn delete(&self, subscribe_id: &SubscribeId, user_id: &UserId) -> Result<(), SubscribeError>;
    }
  }

  fn create_mock_dto() -> SubscribeDto {
    let now = Utc::now();
    SubscribeDto::new(
      SubscribeId::new().to_string(),
      UserId::new().to_string(),
      "Netflix".to_string(),
      PaymentMethodId::new().to_string(),
      "1980".to_string(),
      "MONTHLY".to_string(),
      CategoryId::new().to_string(),
      "/path/to/netflix-icon.png".to_string(),
      true,
      now,
      now + chrono::Duration::days(30),
      true,
      "ACTIVE".to_string(),
      Some("Test subscription".to_string()),
    )
  }

  fn create_mock_domain() -> Subscribe {
        let subscribe_id = SubscribeId::new();
    let user_id = UserId::new();
    let name = SubscribeName::new("hoge").unwrap();
    let payment_method_id = PaymentMethodId::new();
    let amount = Amount::try_from(Decimal::ONE_HUNDRED).unwrap();
    let payment_cycle = PaymentCycle::Monthly;
    let now = Utc::now();
    let category_id = CategoryId::new();
    let icon_path = String::from("/path/to/icon");
    let notification = true;
    let auto_renewal = true;
    let status = SubscribeStatus::ACTIVE;
    let memo = Some("テストメモ".to_owned());

    let subscribe = Subscribe::from(
      subscribe_id.clone(),
      user_id.clone(),
      name.clone(),
      payment_method_id.clone(),
      amount.clone(),
      payment_cycle.clone(),
      category_id.clone(),
      icon_path.clone(),
      notification,
      now,
      now,
      auto_renewal,
      status.clone(),
      memo.clone(),
    );
    subscribe
  }
}
