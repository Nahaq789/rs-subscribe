use crate::dtos::DTO;
use crate::error::ApplicationError;
use chrono::{DateTime, Utc};
use domain::payment::payment_method_id::PaymentMethodId;
use domain::payment::payment_method_name::{PaymentMethodCategoryName, PaymentMethodKindName};
use domain::payment::PaymentMethod;
use domain::user::user_id::UserId;
use domain::AggregateId;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDTO {
  pub payment_method_id: String,
  pub user_id: String,
  pub method_name: String,
  pub method_kind_name: String,
  pub additional_name: String,
  pub created_at: Option<DateTime<Utc>>,
  pub updated_at: Option<DateTime<Utc>>,
}

impl PaymentMethodDTO {
  fn new(
    payment_method_id: &str,
    user_id: &str,
    method_name: &str,
    method_kind_name: &str,
    additional_name: &str,
    created_at: &Option<DateTime<Utc>>,
    updated_at: &Option<DateTime<Utc>>,
  ) -> PaymentMethodDTO {
    PaymentMethodDTO {
      payment_method_id: payment_method_id.to_owned(),
      user_id: user_id.to_owned(),
      method_name: method_name.to_owned(),
      method_kind_name: method_kind_name.to_owned(),
      additional_name: additional_name.to_string(),
      created_at: created_at.to_owned(),
      updated_at: updated_at.to_owned(),
    }
  }
}

impl DTO<PaymentMethodDTO, PaymentMethod, ApplicationError> for PaymentMethodDTO {
  fn map_to_domain_model(v: PaymentMethodDTO) -> Result<PaymentMethod, ApplicationError> {
    let payment_method_id = match &*v.payment_method_id {
      "" => PaymentMethodId::new(),
      _ => PaymentMethodId::from_str(&v.payment_method_id)
        .map_err(|e| ApplicationError::InvalidAggregateIdFormatError(e.to_string()))?,
    };
    let user_id = UserId::from_str(&v.user_id)
      .map_err(|e| ApplicationError::InvalidAggregateIdFormatError(e.to_string()))?;
    let method_name = PaymentMethodCategoryName::from_str(&v.method_name)
      .map_err(|e| ApplicationError::PaymentMethodError(e.to_string()))?;
    let method_kind_name = PaymentMethodKindName::from_str(&v.method_kind_name)
      .map_err(|e| ApplicationError::PaymentMethodError(e.to_string()))?;

    let created_at = PaymentMethod::make_created_at(&v.payment_method_id, v.created_at)
      .map_err(|e| ApplicationError::PaymentMethodError(e.to_string()))?;

    let updated_at = PaymentMethod::make_updated_at(&v.created_at);

    PaymentMethod::is_valid_method_combination(&method_name, &method_kind_name)
      .map_err(|e| ApplicationError::PaymentMethodError(e.to_string()))?;

    let payment_method = PaymentMethod::new(
      payment_method_id,
      user_id,
      method_name,
      method_kind_name,
      &v.additional_name,
      created_at,
      updated_at,
    );

    Ok(payment_method)
  }

  fn map_to_dto(v: &PaymentMethod) -> PaymentMethodDTO {
    let payment_method_id = v.payment_method_id().value();
    let user_id = v.user_id().value();
    let method_name = v.method_name().to_string();
    let method_kind_name = v.method_kind_name().to_string();
    let additional_name = v.additional_name();
    let created_at = Some(v.created_at().to_owned());
    let updated_at = v.updated_at();

    let payment_dto = PaymentMethodDTO::new(
      payment_method_id,
      user_id,
      &method_name,
      &method_kind_name,
      additional_name,
      &created_at,
      updated_at,
    );

    payment_dto
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use domain::payment::payment_method_name::CreditCard;

  #[test]
  fn test_map_to_dto() {
    let payment_method_id = PaymentMethodId::new();
    let user_id = UserId::new();
    let method_name = PaymentMethodCategoryName::CreditCard;
    let method_kind_name = PaymentMethodKindName::CreditCard(CreditCard::JCB);
    let additional_name = "hoge";
    let created_at = Utc::now();
    let updated_at = Some(Utc::now());

    let payment_method = PaymentMethod::new(
      payment_method_id.clone(),
      user_id.clone(),
      method_name.clone(),
      method_kind_name.clone(),
      additional_name,
      created_at.clone(),
      updated_at.clone(),
    );

    let result = PaymentMethodDTO::map_to_dto(&payment_method);

    assert_eq!(&result.payment_method_id, payment_method_id.value());
    assert_eq!(&result.user_id, user_id.value());
    assert_eq!(&result.method_name.to_string(), &method_name.to_string());
    assert_eq!(
      &result.method_kind_name.to_string(),
      &method_kind_name.to_string()
    );
    assert_eq!(&result.additional_name, additional_name);
    assert_eq!(&result.created_at, &Some(created_at));
    assert_eq!(&result.updated_at, &updated_at)
  }

  #[test]
  fn test_map_to_domain_model() {
    // Setup test data
    let payment_method_id = PaymentMethodId::new();
    let user_id = UserId::new();
    let created_at =
      PaymentMethod::make_created_at(payment_method_id.value(), Some(Utc::now())).unwrap();
    let updated_at = PaymentMethod::make_updated_at(&Some(Utc::now()));

    let dto = PaymentMethodDTO {
      payment_method_id: payment_method_id.value().to_string(),
      user_id: user_id.value().to_string(),
      method_name: "Credit Card".to_string(),
      method_kind_name: "JCB".to_string(),
      additional_name: "test_card".to_string(),
      created_at: Some(created_at),
      updated_at,
    };

    // Execute
    let result = PaymentMethodDTO::map_to_domain_model(dto).unwrap();
    println!("{:?}", result.updated_at());
    println!("{:?}", updated_at);
    // Assert
    assert_eq!(
      result.payment_method_id().value(),
      payment_method_id.value()
    );
    assert_eq!(result.user_id().value(), user_id.value());
    assert_eq!(result.method_name().to_string(), "Credit Card");
    assert_eq!(result.method_kind_name().to_string(), "JCB");
    assert_eq!(result.additional_name(), "test_card");
    assert_eq!(result.created_at(), &created_at);
    assert_eq!(
      result.updated_at().unwrap().timestamp(),
      updated_at.unwrap().timestamp()
    );
  }

  #[test]
  fn test_map_to_domain_model_with_invalid_id() {
    // Setup test data with invalid ID
    let dto = PaymentMethodDTO {
      payment_method_id: "invalid_id".to_string(),
      user_id: UserId::new().value().to_string(),
      method_name: "CreditCard".to_string(),
      method_kind_name: "JCB".to_string(),
      additional_name: "test_card".to_string(),
      created_at: Some(Utc::now()),
      updated_at: None,
    };

    // Execute and assert error
    let result = PaymentMethodDTO::map_to_domain_model(dto);
    assert!(result.is_err());
    assert!(matches!(
      result.unwrap_err(),
      ApplicationError::InvalidAggregateIdFormatError(_)
    ));
  }

  #[test]
  fn test_map_to_domain_model_with_invalid_method_name() {
    // Setup test data with invalid method name
    let dto = PaymentMethodDTO {
      payment_method_id: PaymentMethodId::new().value().to_string(),
      user_id: UserId::new().value().to_string(),
      method_name: "InvalidMethod".to_string(),
      method_kind_name: "JCB".to_string(),
      additional_name: "test_card".to_string(),
      created_at: Some(Utc::now()),
      updated_at: None,
    };

    // Execute and assert error
    let result = PaymentMethodDTO::map_to_domain_model(dto);
    assert!(result.is_err());
    assert!(matches!(
      result.unwrap_err(),
      ApplicationError::PaymentMethodError(_)
    ));
  }
}
