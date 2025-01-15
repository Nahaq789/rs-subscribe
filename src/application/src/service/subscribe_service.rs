pub struct SubscribeServiceImpl<T: domain::repository::subscribe_repository::SubscribeRepository> {
  repository: T,
}

impl<T: domain::repository::subscribe_repository::SubscribeRepository> SubscribeServiceImpl<T> {
  pub fn new(repository: T) -> SubscribeServiceImpl<T> {
    Self { repository }
  }
}

impl<T: domain::repository::subscribe_repository::SubscribeRepository>
  crate::service::SubscribeService for SubscribeServiceImpl<T>
{
  fn create_subscribe(
    &self,
    subscribe: crate::dtos::subscribe_dto::SubscribeDto,
  ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + '_>> {
    let result = Box::pin(async move {});
    todo!()
  }
}
