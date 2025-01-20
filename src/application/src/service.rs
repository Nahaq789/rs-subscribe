use crate::dtos;
use crate::dtos::payment_method_dto::PaymentMethodDTO;
use crate::error::ApplicationError;

pub mod payment_method_service;
pub mod subscribe_service;

#[async_trait::async_trait]
pub trait PaymentMethodService: Send + Sync {
    async fn create_payment_method(&self, payment: PaymentMethodDTO) -> Result<(), ApplicationError>;
    async fn find_payment_method_all(&self, user_id: &str) -> Result<Vec<PaymentMethodDTO>, ApplicationError>;
    async fn find_payment_method_by_id(
        &self,
        payment_id: &str,
        user_id: &str,
    ) -> Result<PaymentMethodDTO, ApplicationError>;
    async fn update_payment_method(&self, payment: PaymentMethodDTO) -> Result<(), ApplicationError>;
    async fn delete_payment_method(&self, payment_id: &str, user_id: &str) -> Result<(), ApplicationError>;
}

pub trait SubscribeService: Send + Sync {
    fn create_subscribe(
        &self,
        subscribe: dtos::subscribe_dto::SubscribeDto,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + '_>>;

    fn find_subscribe_all<'a>(
        &'a self,
        user_id: &'a str,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = anyhow::Result<Vec<crate::dtos::subscribe_dto::SubscribeDto>>> + Send + '_,
        >,
    >;

    fn find_subscribe_by_id<'a>(
        &'a self,
        user_id: &'a str,
        subscribe_id: &'a str,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = anyhow::Result<crate::dtos::subscribe_dto::SubscribeDto>> + Send + '_>,
    >;

    fn update_subscribe(
        &self,
        subscribe: dtos::subscribe_dto::SubscribeDto,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + '_>>;

    fn delete_subscribe<'a>(
        &'a self,
        user_id: &'a str,
        subscribe_id: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + '_>>;
}
