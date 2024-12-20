use std::str::FromStr;

use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_sdk_dynamodb::types::AttributeValue;
use domain::{
  repository::subscribe_repository::SubscribeRepository,
  subscribe::{
    subscribe_error::SubscribeError, subscribe_id::SubscribeId, subscribe_name::SubscribeName,
    Subscribe,
  },
  user::user_id::UserId,
  value_object::amount::Amount,
  AggregateId,
};
use tracing::{error, info};

use crate::mapper::{as_string, Mapper};

const SUBSCRIBE_KEY: &str = "subscribe_id";
const USER_ID: &str = "user_id";

const NAME: &str = "name";
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
    todo!()
  }

  async fn find_all(&self, user_id: &UserId) -> Result<Vec<Subscribe>, SubscribeError> {
    todo!()
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
  }
}
