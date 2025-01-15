use std::str::FromStr;

use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_sdk_dynamodb::types::AttributeValue;
use domain::{
  category::category_id::CategoryId,
  payment::payment_method_id::PaymentMethodId,
  payment_cycle::PaymentCycle,
  repository::subscribe_repository::SubscribeRepository,
  subscribe::{
    subscribe_error::SubscribeError, subscribe_id::SubscribeId, subscribe_name::SubscribeName,
    subscribe_status::SubscribeStatus, Subscribe,
  },
  user::user_id::UserId,
  value_object::amount::Amount,
  AggregateId,
};
use tracing::{error, info};

use crate::mapper::{as_datetime, as_string, Mapper};

const SUBSCRIBE_KEY: &str = "subscribe_id";
const USER_ID: &str = "user_id";

const NAME: &str = "name";
const PAYMENT_METHOD_ID: &str = "payment_method_id";
const AMOUNT: &str = "amount";
const PAYMENT_CYCLE: &str = "payment_cycle";
const CATEGORY_ID: &str = "category_id";
const ICON_LOCAL_PATH: &str = "icon_local_path";
const NOTIFICATION: &str = "notification";
const FIRST_PAYMENT_DATE: &str = "first_payment_date";
const NEXT_PAYMENT_DATE: &str = "next_payment_date";
const AUTO_RENEWAL: &str = "auto_renewal";
const STATUS: &str = "status";
const MEMO: &str = "memo";

const USER_ID_CONDITION: &str = "#user_id = :user_id";
const USER_ID_ATTR: &str = "#user_id";
const USER_ID_VALUE: &str = ":user_id";

const UPDATE_EXPRESSION: &str = "SET #name = :name, \
                                 #payment_method_id = :payment_method_id, \
                                 #amount = :amount, \
                                 #payment_cycle = :payment_cycle, \
                                 #category_id = :category_id, \
                                 #icon_local_path = :icon_local_path, \
                                 #notification = :notification, \
                                 #first_payment_date = :first_payment_date, \
                                 #next_payment_date = :next_payment_date, \
                                 #auto_renewal = :auto_renewal, \
                                 #status = :status, \
                                 #memo = :memo";

const NAME_ATTR: &str = "#name";
const PAYMENT_METHOD_ID_ATTR: &str = "#payment_method_id";
const AMOUNT_ATTR: &str = "#amount";
const PAYMENT_CYCLE_ATTR: &str = "#payment_cycle";
const CATEGORY_ID_ATTR: &str = "#category_id";
const ICON_LOCAL_PATH_ATTR: &str = "#icon_local_path";
const NOTIFICATION_ATTR: &str = "#notification";
const FIRST_PAYMENT_DATE_ATTR: &str = "#first_payment_date";
const NEXT_PAYMENT_DATE_ATTR: &str = "#next_payment_date";
const AUTO_RENEWAL_ATTR: &str = "#auto_renewal";
const STATUS_ATTR: &str = "#status";
const MEMO_ATTR: &str = "#memo";

const NAME_VALUE: &str = ":name";
const PAYMENT_METHOD_ID_VALUE: &str = ":payment_method_id";
const AMOUNT_VALUE: &str = ":amount";
const PAYMENT_CYCLE_VALUE: &str = ":payment_cycle";
const CATEGORY_ID_VALUE: &str = ":category_id";
const ICON_LOCAL_PATH_VALUE: &str = ":icon_local_path";
const NOTIFICATION_VALUE: &str = ":notification";
const FIRST_PAYMENT_DATE_VALUE: &str = ":first_payment_date";
const NEXT_PAYMENT_DATE_VALUE: &str = ":next_payment_date";
const AUTO_RENEWAL_VALUE: &str = ":auto_renewal";
const STATUS_VALUE: &str = ":status";
const MEMO_VALUE: &str = ":memo";

#[derive(Debug)]
pub struct SubscribeRepositoryImpl {
  client: aws_sdk_dynamodb::Client,
  table: String,
}

impl SubscribeRepositoryImpl {
  pub fn new(client: aws_sdk_dynamodb::Client, table: &str) -> Self {
    Self {
      client,
      table: table.to_owned(),
    }
  }
}

#[async_trait::async_trait]
impl SubscribeRepository for SubscribeRepositoryImpl {
  async fn create(&self, subscribe: &Subscribe) -> Result<(), SubscribeError> {
    let request = self
      .client
      .put_item()
      .table_name(&self.table)
      .item(
        SUBSCRIBE_KEY,
        AttributeValue::S(subscribe.subscribe_id().to_string()),
      )
      .item(USER_ID, AttributeValue::S(subscribe.user_id().to_string()))
      .item(NAME, AttributeValue::S(subscribe.name().to_string()))
      .item(PAYMENT_METHOD_ID, AttributeValue::S(subscribe.payment_method_id().to_string()))
      .item(AMOUNT, AttributeValue::S(subscribe.amount().to_string()))
      .item(PAYMENT_CYCLE, AttributeValue::S(subscribe.payment_cycle().to_string()))
      .item(CATEGORY_ID, AttributeValue::S(subscribe.category_id().to_string()))
      .item(ICON_LOCAL_PATH, AttributeValue::S(subscribe.icon_local_path().to_string()))
      .item(NOTIFICATION, AttributeValue::Bool(subscribe.notification()))
      .item(FIRST_PAYMENT_DATE, AttributeValue::S(subscribe.first_payment_date().to_string()))
      .item(NEXT_PAYMENT_DATE, AttributeValue::S(subscribe.next_payment_date().to_string()))
      .item(AUTO_RENEWAL, AttributeValue::Bool(subscribe.auto_renewal()))
      .item(STATUS, AttributeValue::S(subscribe.status().to_string()))
      .item(MEMO, AttributeValue::S(subscribe.memo().to_string()));

    match request.send().await {
      Ok(p) => {
        info!("{:?}", p);
        Ok(())
      }
      Err(e) => {
        error!("{:?}", e);
        Err(SubscribeError::CreateSubscribeFailed(e.to_string()))
      }
    }
  }

  async fn find_all(&self, user_id: &UserId) -> Result<Vec<Subscribe>, SubscribeError> {
    let result = self.client
      .query()
      .table_name(&self.table)
      .key_condition_expression(USER_ID_CONDITION)
      .expression_attribute_names(USER_ID_ATTR, USER_ID)
      .expression_attribute_values(USER_ID_VALUE, AttributeValue::S(user_id.to_string()))
      .send()
      .await
      .map_err(|e| {
        let msg = match e.message() {
          Some(s) => s.to_string(),
          None => e.to_string()
        };
        SubscribeError::QueryError(msg)
      })?;
    
    match result.items {
        Some(items) => {
          info!("{:?}", items);
          let result = items
            .into_iter()
            .map(|item| SubscribeRepositoryImpl::map_to_domain_model(item))
            .collect();
          result
        },
        None => {
          Ok(vec![])
        }
    }
  }

  async fn find_by_id(
    &self,
    subscribe_id: &SubscribeId,
    user_id: &UserId,
  ) -> Result<Subscribe, SubscribeError> {
    let result = self
      .client
      .get_item()
      .table_name(&self.table)
      .key(
        SUBSCRIBE_KEY,
        AttributeValue::S(subscribe_id.value().to_owned()),
      )
      .key(USER_ID, AttributeValue::S(user_id.value().to_owned()))
      .send()
      .await
      .map_err(|e| {
        let msg = match e.message() {
          Some(s) => s.to_string(),
          None => e.to_string(),
        };
        SubscribeError::FindByIdError(msg)
      })?;

    match result.item {
      Some(item) => {
        info!("{:?}", item);
        SubscribeRepositoryImpl::map_to_domain_model(item)
      }
      None => {
        let error = SubscribeError::FindByIdError(format!(
          "subscribe_id: {:?}, user_id: {:?}",
          subscribe_id, user_id
        ));
        error!("{:?}", error);
        Err(error)
      }
    }
  }

  async fn update(&self, subscribe: &Subscribe) -> Result<(), SubscribeError> {
    let result = self
      .client
      .update_item()
      .table_name(&self.table)
      .key(
        SUBSCRIBE_KEY,
        AttributeValue::S(subscribe.subscribe_id().value().to_owned()),
      )
      .key(
        USER_ID,
        AttributeValue::S(subscribe.user_id().value().to_owned()),
      )
      .update_expression(UPDATE_EXPRESSION)
      .expression_attribute_names(NAME_ATTR, NAME_VALUE)
      .expression_attribute_names(PAYMENT_METHOD_ID_ATTR, PAYMENT_METHOD_ID_VALUE)
      .expression_attribute_names(AMOUNT_ATTR, AMOUNT_VALUE)
      .expression_attribute_names(PAYMENT_CYCLE_ATTR, PAYMENT_CYCLE_VALUE)
      .expression_attribute_names(CATEGORY_ID_ATTR, CATEGORY_ID_VALUE)
      .expression_attribute_names(ICON_LOCAL_PATH_ATTR, ICON_LOCAL_PATH_VALUE)
      .expression_attribute_names(NOTIFICATION_ATTR, NOTIFICATION_VALUE)
      .expression_attribute_names(FIRST_PAYMENT_DATE_ATTR, FIRST_PAYMENT_DATE_VALUE)
      .expression_attribute_names(NEXT_PAYMENT_DATE_ATTR, NEXT_PAYMENT_DATE_VALUE)
      .expression_attribute_names(AUTO_RENEWAL_ATTR, AUTO_RENEWAL_VALUE)
      .expression_attribute_names(STATUS_ATTR, STATUS_VALUE)
      .expression_attribute_names(MEMO_ATTR, MEMO_VALUE)
      .expression_attribute_values(NAME, AttributeValue::S(subscribe.name().to_string()))
      .expression_attribute_values(
        PAYMENT_METHOD_ID,
        AttributeValue::S(subscribe.payment_method_id().to_string()),
      )
      .expression_attribute_values(AMOUNT, AttributeValue::S(subscribe.amount().to_string()))
      .expression_attribute_values(
        PAYMENT_CYCLE,
        AttributeValue::S(subscribe.payment_cycle().as_str().to_owned()),
      )
      .expression_attribute_values(
        CATEGORY_ID,
        AttributeValue::S(subscribe.category_id().to_string()),
      )
      .expression_attribute_values(
        ICON_LOCAL_PATH,
        AttributeValue::S(subscribe.icon_local_path().to_string()),
      )
      .expression_attribute_values(
        NOTIFICATION,
        AttributeValue::S(subscribe.notification().to_string()),
      )
      .expression_attribute_values(
        FIRST_PAYMENT_DATE,
        AttributeValue::S(subscribe.first_payment_date().to_rfc3339()),
      )
      .expression_attribute_values(
        NEXT_PAYMENT_DATE,
        AttributeValue::S(subscribe.next_payment_date().to_rfc3339()),
      )
      .expression_attribute_values(
        AUTO_RENEWAL,
        AttributeValue::S(subscribe.auto_renewal().to_string()),
      )
      .expression_attribute_values(STATUS, AttributeValue::S(subscribe.status().to_string()))
      .expression_attribute_values(MEMO, AttributeValue::S(subscribe.memo().to_owned()))
      .send()
      .await
      .map_err(|e| {
        let msg = match e.message() {
          Some(s) => s.to_string(),
          None => e.to_string(),
        };
        SubscribeError::UpdateSubscribeError(msg)
      });

    match result {
      Ok(v) => {
        info!("{:?}", v);
        Ok(())
      }
      Err(e) => {
        error!("{:?}", e);
        Err(SubscribeError::UpdateSubscribeError(e.to_string()))
      }
    }
  }

  async fn delete(
    &self,
    subscribe_id: &SubscribeId,
    user_id: &UserId,
  ) -> Result<(), SubscribeError> {
    let result = self
      .client
      .delete_item()
      .table_name(&self.table)
      .key(
        SUBSCRIBE_KEY,
        AttributeValue::S(subscribe_id.value().to_owned()),
      )
      .key(USER_ID, AttributeValue::S(user_id.value().to_owned()))
      .send()
      .await
      .map_err(|e| {
        let msg = match e.message() {
          Some(s) => s.to_string(),
          None => e.to_string(),
        };
        SubscribeError::DeleteSubscribeFailed(msg)
      });

    match result {
      Ok(u) => {
        info!("{:?}", u);
        Ok(())
      }
      Err(e) => {
        error!("{:?}", e);
        Err(SubscribeError::DeleteSubscribeFailed(e.to_string()))
      }
    }
  }
}

impl Mapper<Subscribe, SubscribeError> for SubscribeRepositoryImpl {
  fn map_to_domain_model(
    v: std::collections::HashMap<String, AttributeValue>,
  ) -> Result<Subscribe, SubscribeError> {
    let subscribe_id = SubscribeId::from_str(&as_string(v.get(SUBSCRIBE_KEY), ""))?;
    let user_id = UserId::from_str(&as_string(v.get(USER_ID), ""))?;
    let name = SubscribeName::from_str(&as_string(v.get(NAME), ""))?;
    let payment_method_id = PaymentMethodId::from_str(&as_string(v.get(PAYMENT_METHOD_ID), ""))?;
    let amount = Amount::from_str(&as_string(v.get(AMOUNT), ""))?;
    let payment_cycle = PaymentCycle::from_str(&as_string(v.get(PAYMENT_CYCLE), ""))?;
    let category_id = CategoryId::from_str(&as_string(v.get(CATEGORY_ID), ""))?;
    let icon_local_path = as_string(v.get(ICON_LOCAL_PATH), "");
    let notification = v
      .get(NOTIFICATION)
      .ok_or(SubscribeError::MissingField(NOTIFICATION.into()))?
      .as_bool()
      .map_err(|_| SubscribeError::ParseFailed(NOTIFICATION.into()))?;
    let first_payment_date = as_datetime(v.get(FIRST_PAYMENT_DATE))
      .ok_or(SubscribeError::MissingField(FIRST_PAYMENT_DATE.to_string()))?;
    let next_payment_date = as_datetime(v.get(NEXT_PAYMENT_DATE))
      .ok_or(SubscribeError::MissingField(NEXT_PAYMENT_DATE.to_string()))?;
    let auto_renewal = v
      .get(AUTO_RENEWAL)
      .ok_or(SubscribeError::MissingField(AUTO_RENEWAL.into()))?
      .as_bool()
      .map_err(|_| SubscribeError::ParseFailed(AUTO_RENEWAL.into()))?;
    let status = SubscribeStatus::from_str(&as_string(v.get(STATUS), ""))?;
    let memo = as_string(v.get(MEMO), "");

    Ok(Subscribe::from(
      subscribe_id,
      user_id,
      name,
      payment_method_id,
      amount,
      payment_cycle,
      category_id,
      icon_local_path,
      *notification,
      first_payment_date,
      next_payment_date,
      *auto_renewal,
      status,
      &memo,
    ))
  }
}

#[cfg(test)]
mod tests {
  use chrono::Utc;
  use domain::category::category_id;
  use std::collections::HashMap;

  use super::*;

  #[test]
  fn test_map_to_domain_model_success() {
    let test_case = vec![HashMap::from([
      (
        SUBSCRIBE_KEY.into(),
        AttributeValue::S(SubscribeId::new().to_string()),
      ),
      (USER_ID.into(), AttributeValue::S(UserId::new().to_string())),
      (NAME.to_string(), AttributeValue::S("hoge".into())),
      (
        PAYMENT_METHOD_ID.into(),
        AttributeValue::S(PaymentMethodId::new().to_string()),
      ),
      (AMOUNT.into(), AttributeValue::S("5000".into())),
      (PAYMENT_CYCLE.into(), AttributeValue::S("monthly".into())),
      (
        CATEGORY_ID.into(),
        AttributeValue::S(category_id::CategoryId::new().to_string()),
      ),
      (ICON_LOCAL_PATH.into(), AttributeValue::S("../../".into())),
      (NOTIFICATION.into(), AttributeValue::Bool(true)),
      (
        FIRST_PAYMENT_DATE.into(),
        AttributeValue::S(Utc::now().to_rfc3339()),
      ),
      (
        NEXT_PAYMENT_DATE.into(),
        AttributeValue::S(Utc::now().to_rfc3339()),
      ),
      (AUTO_RENEWAL.into(), AttributeValue::Bool(false)),
      (STATUS.into(), AttributeValue::S("ACTIVE".into())),
      (MEMO.into(), AttributeValue::S("hoge".into())),
    ])];

    let _test = test_case
      .into_iter()
      .map(
        |test| match SubscribeRepositoryImpl::map_to_domain_model(test.clone()) {
          Ok(v) => {
            assert_eq!(
              v.subscribe_id().to_string(),
              as_string(test.get(SUBSCRIBE_KEY), "")
            );
            assert_eq!(v.user_id().to_string(), as_string(test.get(USER_ID), ""));
            assert_eq!(v.name().to_string(), as_string(test.get(NAME), ""));
            assert_eq!(
              v.payment_method_id().to_string(),
              as_string(test.get(PAYMENT_METHOD_ID), "")
            );
            assert_eq!(v.amount().to_string(), as_string(test.get(AMOUNT), ""));
            assert_eq!(
              v.payment_cycle().to_string(),
              as_string(test.get(PAYMENT_CYCLE), "")
            );
            assert_eq!(
              v.category_id().to_string(),
              as_string(test.get(CATEGORY_ID), "")
            );
            assert_eq!(
              v.icon_local_path().to_string(),
              as_string(test.get(ICON_LOCAL_PATH), "")
            );
            assert_eq!(
              v.notification(),
              *test.get(NOTIFICATION).unwrap().as_bool().unwrap()
            );
            assert_eq!(
              v.first_payment_date().to_rfc3339(),
              as_string(test.get(FIRST_PAYMENT_DATE), "")
            );
            assert_eq!(
              v.next_payment_date().to_rfc3339(),
              as_string(test.get(NEXT_PAYMENT_DATE), "")
            );
            assert_eq!(
              v.auto_renewal(),
              *test.get(AUTO_RENEWAL).unwrap().as_bool().unwrap()
            );
            assert_eq!(v.status().to_string(), as_string(test.get(STATUS), ""));
            assert_eq!(v.memo().to_string(), as_string(test.get(MEMO), ""));
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
