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
