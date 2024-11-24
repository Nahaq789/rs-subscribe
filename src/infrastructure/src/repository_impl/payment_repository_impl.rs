use aws_sdk_dynamodb::types::AttributeValue;
use domain::payment::payment_error::PaymentError;
use domain::payment::payment_method_id::PaymentMethodId;
use domain::payment::PaymentMethod;
use domain::repository::payment_repository::PaymentRepository;
use domain::user::user_id::UserId;
use domain::AggregateId;

const PAYMENT_METHOD_KEY: &str = "payment_method_id";
const USER_ID: &str = "user_id";
const METHOD_NAME: &str = "method_name";
const ADDITIONAL_NAME: &str = "additional_name";
const CREATED_AT: &str = "created_at";
const UPDATED_AT: &str = "updated_at";

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
  ) -> Result<(), PaymentError> {
    let request = self
      .client
      .put_item()
      .table_name(table)
      .item(
        PAYMENT_METHOD_KEY,
        AttributeValue::S(payment.payment_method_id().value().to_string()),
      )
      .item(USER_ID, AttributeValue::S(user_id.value().to_string()))
      .item(
        METHOD_NAME,
        AttributeValue::S(payment.method_name().category().to_string()),
      )
      .item(
        ADDITIONAL_NAME,
        AttributeValue::S(payment.additional_name().to_string()),
      )
      .item(
        CREATED_AT,
        match payment.created_at() {
          Some(v) => AttributeValue::S(v.to_rfc3339()),
          None => AttributeValue::Null(true),
        },
      )
      .item(
        UPDATED_AT,
        match payment.updated_at() {
          Some(v) => AttributeValue::S(v.to_rfc3339()),
          None => AttributeValue::Null(true),
        },
      );

    match request.send().await {
      Ok(_) => Ok(()),
      Err(e) => Err(PaymentError::CreatePaymentMethodFailed(e.to_string())),
    }
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
      Err(e) => Err(PaymentError::DeletePaymentMethodFailed(e.to_string())),
    }
  }
}
