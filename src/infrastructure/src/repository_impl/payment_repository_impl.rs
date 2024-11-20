use aws_sdk_dynamodb::types::AttributeValue;
use domain::payment::payment_error::PaymentError;
use domain::payment::payment_method_id::PaymentMethodId;
use domain::payment::PaymentMethod;
use domain::repository::payment_repository::PaymentRepository;
use domain::user::user_id::UserId;
use domain::AggregateId;

use crate::entity::mapper::payment_mapper::PaymentMapper;

pub struct PaymentRepositoryImpl {
  client: aws_sdk_dynamodb::Client,
}

impl PaymentRepositoryImpl {
  pub fn new(client: aws_sdk_dynamodb::Client) -> Self {
    Self { client }
  }
}

#[async_trait::async_trait]
impl PaymentRepository for PaymentRepositoryImpl {
  async fn create(
    &self,
    payment: &PaymentMethod,
    user_id: &UserId,
    table: &str,
  ) -> Result<PaymentMethod, PaymentError> {
    let request = self
      .client
      .put_item()
      .table_name(table)
      .item(
        entity.payment_id().0,
        AttributeValue::S(*entity.payment_id().1),
      )
      .send()
      .await;
  }

  async fn find_all(
    &self,
    user_id: &UserId,
    table: &str,
  ) -> Result<Vec<PaymentMethod>, PaymentError> {
    todo!()
  }

  async fn find_by_id(
    &self,
    payment_id: &PaymentMethodId,
    table: &str,
  ) -> Result<PaymentMethod, PaymentError> {
    todo!()
  }

  async fn update(
    &self,
    payment: &PaymentMethod,
    table: &str,
  ) -> Result<PaymentMethod, PaymentError> {
    todo!()
  }

  async fn delete(
    &self,
    payment_id: &PaymentMethodId,
    table: &str,
    key: &str,
  ) -> Result<(), PaymentError> {
    match self
      .client
      .delete_item()
      .table_name(table)
      .key(key, AttributeValue::S(payment_id.value().to_owned()))
      .send()
      .await
    {
      Ok(out) => {
        println!("delete item: {:?}", out);
        Ok(())
      }
      Err(e) => Err(PaymentError::InvalidDeleteItem(e.to_string())),
    }
  }
}
