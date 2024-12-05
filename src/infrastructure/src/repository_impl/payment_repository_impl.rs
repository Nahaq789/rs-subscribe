use crate::mapper::{as_datetime, as_string, Mapper};
use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_sdk_dynamodb::types::AttributeValue;
use domain::payment::payment_error::PaymentError;
use domain::payment::payment_method_id::PaymentMethodId;
use domain::payment::payment_method_name::{PaymentMethodCategoryName, PaymentMethodKindName};
use domain::payment::PaymentMethod;
use domain::repository::payment_repository::PaymentRepository;
use domain::user::user_id::UserId;
use domain::AggregateId;
use std::collections::HashMap;
use std::str::FromStr;

const PAYMENT_METHOD_KEY: &str = "payment_method_id";
const USER_ID: &str = "user_id";
const METHOD_NAME: &str = "method_name";
const METHOD_KIND_NAME: &str = "method_kind_name";
const ADDITIONAL_NAME: &str = "additional_name";
const CREATED_AT: &str = "created_at";
const UPDATED_AT: &str = "updated_at";

const USER_ID_CONDITION: &str = "#user_id = :user_id";
const USER_ID_ATTR: &str = "#user_id";
const USER_ID_VALUE: &str = ":user_id";

// Expression constants
const UPDATE_EXPRESSION: &str = "SET #method_name = :method_name, \
                                   #method_kind_name = :method_kind_name, \
                                   #additional_name = :additional_name, \
                                   #updated_at = :updated_at";

// Attribute name constants
const METHOD_NAME_ATTR: &str = "#method_name";
const METHOD_KIND_NAME_ATTR: &str = "#method_kind_name";
const ADDITIONAL_NAME_ATTR: &str = "#additional_name";
const UPDATED_AT_ATTR: &str = "#updated_at";

// Attribute value constants
const METHOD_NAME_VALUE: &str = ":method_name";
const METHOD_KIND_NAME_VALUE: &str = ":method_kind_name";
const ADDITIONAL_NAME_VALUE: &str = ":additional_name";
const UPDATED_AT_VALUE: &str = ":updated_at";

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
  async fn create(&self, payment: &PaymentMethod, table: &str) -> Result<(), PaymentError> {
    let request = self
      .client
      .put_item()
      .table_name(table)
      .item(
        PAYMENT_METHOD_KEY,
        AttributeValue::S(payment.payment_method_id().value().to_string()),
      )
      .item(
        USER_ID,
        AttributeValue::S(payment.user_id().value().to_string()),
      )
      .item(
        METHOD_NAME,
        AttributeValue::S(payment.method_name().to_string()),
      )
      .item(
        METHOD_KIND_NAME,
        AttributeValue::S(payment.method_kind_name().to_string()),
      )
      .item(
        ADDITIONAL_NAME,
        AttributeValue::S(payment.additional_name().to_string()),
      )
      .item(
        CREATED_AT,
        AttributeValue::S(payment.created_at().to_rfc3339()),
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
    let result = self
      .client
      .query()
      .table_name(table)
      .key_condition_expression(USER_ID_CONDITION)
      .expression_attribute_names(USER_ID_ATTR, USER_ID)
      .expression_attribute_values(
        USER_ID_VALUE,
        AttributeValue::S(user_id.value().to_string()),
      )
      .send()
      .await
      .map_err(|e| {
        let msg = match e.message() {
          Some(s) => s.to_string(),
          None => e.to_string()
        };
        PaymentError::QueryError(msg)
      })?;

    match result.items {
      Some(items) => {
        let result = items
          .into_iter()
          .map(|item| PaymentRepositoryImpl::map_to_domain_model(item))
          .collect();
        result
      }
      None => Ok(vec![]),
    }
  }

  async fn find_by_id(
    &self,
    payment_id: &PaymentMethodId,
    user_id: &UserId,
    table: &str,
  ) -> Result<PaymentMethod, PaymentError> {
    let result = self
      .client
      .get_item()
      .table_name(table)
      .key(
        PAYMENT_METHOD_KEY,
        AttributeValue::S(payment_id.value().to_string()),
      )
      .key(USER_ID, AttributeValue::S(user_id.value().to_string()))
      .send()
      .await
      .map_err(|e| PaymentError::FindByIdError(e.to_string()))?;

    match result.item {
      Some(item) => PaymentRepositoryImpl::map_to_domain_model(item),
      None => Err(PaymentError::FindByIdError(payment_id.value().to_string())),
    }
  }

  async fn update(&self, payment: &PaymentMethod, table: &str) -> Result<(), PaymentError> {
    let result = self
      .client
      .update_item()
      .table_name(table)
      .key(
        PAYMENT_METHOD_KEY,
        AttributeValue::S(payment.payment_method_id().value().to_string()),
      )
      .update_expression(UPDATE_EXPRESSION)
      .expression_attribute_names(METHOD_NAME_ATTR, METHOD_NAME)
      .expression_attribute_names(METHOD_KIND_NAME_ATTR, METHOD_KIND_NAME)
      .expression_attribute_names(ADDITIONAL_NAME_ATTR, ADDITIONAL_NAME)
      .expression_attribute_names(UPDATED_AT_ATTR, UPDATED_AT)
      .expression_attribute_values(
        METHOD_NAME_VALUE,
        AttributeValue::S(payment.method_name().to_string()),
      )
      .expression_attribute_values(
        METHOD_KIND_NAME_VALUE,
        AttributeValue::S(payment.method_kind_name().to_string()),
      )
      .expression_attribute_values(
        ADDITIONAL_NAME_VALUE,
        AttributeValue::S(payment.additional_name().to_string()),
      )
      .expression_attribute_values(
        UPDATED_AT_VALUE,
        match payment.updated_at() {
          Some(v) => AttributeValue::S(v.to_rfc3339()),
          None => AttributeValue::Null(true),
        },
      );

    match result.send().await {
      Ok(_) => Ok(()),
      Err(e) => Err(PaymentError::UpdatePaymentMethodError(e.to_string())),
    }
  }

  async fn delete(
    &self,
    payment_id: &PaymentMethodId,
    user_id: &UserId,
    table: &str,
  ) -> Result<(), PaymentError> {
    match self
      .client
      .delete_item()
      .table_name(table)
      .key(
        PAYMENT_METHOD_KEY,
        AttributeValue::S(payment_id.value().to_owned()),
      )
      .key(USER_ID, AttributeValue::S(user_id.value().to_owned()))
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

impl Mapper<PaymentMethod, PaymentError> for PaymentRepositoryImpl {
  fn map_to_domain_model(
    v: HashMap<String, AttributeValue>,
  ) -> Result<PaymentMethod, PaymentError> {
    let payment_method_id = PaymentMethodId::from_str(&as_string(v.get(PAYMENT_METHOD_KEY), ""))?;
    let user_id = UserId::from_str(&as_string(v.get(USER_ID), ""))?;
    let method_name = PaymentMethodCategoryName::from_str(&as_string(v.get(METHOD_NAME), ""))?;
    let method_kind_name =
      PaymentMethodKindName::from_str(&as_string(v.get(METHOD_KIND_NAME), ""))?;
    let additional_name = &as_string(v.get(ADDITIONAL_NAME), "");
    let created_at =
      as_datetime(v.get(CREATED_AT)).ok_or(PaymentError::MissingField("created_at".to_string()))?;
    let updated_at = as_datetime(v.get(UPDATED_AT));

    let payment = PaymentMethod::new(
      payment_method_id,
      user_id,
      method_name,
      method_kind_name,
      additional_name,
      created_at,
      updated_at,
    );

    Ok(payment)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_to_domain_model() {
    let test_case = vec![HashMap::from([
      (
        PAYMENT_METHOD_KEY.to_string(),
        AttributeValue::S("pay_550e8400-e29b-41d4-a716-446655440000".to_string()),
      ),
      (
        USER_ID.to_string(),
        AttributeValue::S("usr_550e8400-e29b-41d4-a716-446655440000".to_string()),
      ),
      (
        METHOD_NAME.to_string(),
        AttributeValue::S("Credit Card".to_string()),
      ),
      (
        METHOD_KIND_NAME.to_string(),
        AttributeValue::S("JCB".to_string()),
      ),
      (
        CREATED_AT.to_string(),
        AttributeValue::S("2024-01-01T00:00:00Z".to_string()),
      ),
      (
        UPDATED_AT.to_string(),
        AttributeValue::S("2077-02-02T00:00:00Z".to_string()),
      ),
    ])];

    let _ = test_case
      .into_iter()
      .map(
        |test| match PaymentRepositoryImpl::map_to_domain_model(test.clone()) {
          Ok(v) => {
            assert_eq!(
              v.payment_method_id().value().to_string(),
              as_string(test.get(PAYMENT_METHOD_KEY), "")
            );
            assert_eq!(
              v.user_id().value().to_string(),
              as_string(test.get(USER_ID), "")
            );
            assert_eq!(
              v.method_name().to_string(),
              as_string(test.get(METHOD_NAME), "")
            );
            assert_eq!(
              v.method_kind_name().to_string(),
              as_string(test.get(METHOD_KIND_NAME), "")
            )
          }
          Err(e) => {
            println!("{:?}", e.to_string());
            assert!(false)
          }
        },
      )
      .collect::<()>();
  }
}
