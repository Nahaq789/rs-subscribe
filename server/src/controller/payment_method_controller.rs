use crate::app_state::PaymentMethodState;
use application::dtos::payment_method_dto::PaymentMethodDTO;
use application::error::ApplicationError;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde_json::json;
use std::fmt;
use std::fmt::Formatter;

use super::params::payment_method_params::{FindAllParam, FindByIdParams};

pub struct ApplicationErrorWrapper(ApplicationError);

impl fmt::Display for ApplicationErrorWrapper {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0.to_string())
  }
}

impl IntoResponse for ApplicationErrorWrapper {
  fn into_response(self) -> Response {
    let (status, message) = match self {
      ApplicationErrorWrapper(ApplicationError::PaymentMethodError(..)) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
      _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
    };
    let res = json!({
        "message": message,
        "status code": status.as_u16()
    });
    (status, Json(res)).into_response()
  }
}

pub async fn create_payment_method(
  Extension(module): Extension<PaymentMethodState>,
  Json(payload): Json<PaymentMethodDTO>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
  let result = module.state.create_payment_method(payload).await;
  let response = json!({
      "message": "payment method created",
      "status code": StatusCode::OK.as_u16()
  });

  match result {
    Ok(_) => Ok((StatusCode::OK, Json(response))),
    Err(e) => Err(ApplicationErrorWrapper(e)),
  }
}

pub async fn find_payment_method_all(
  Extension(module): Extension<PaymentMethodState>,
  Query(FindAllParam { user_id }): Query<FindAllParam>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
  let result = module.state.find_payment_method_all(&user_id).await;

  match result {
    Ok(v) => Ok((StatusCode::OK, Json(v))),
    Err(e) => Err(ApplicationErrorWrapper(e)),
  }
}

pub async fn find_payment_method_by_id(
  Extension(module): Extension<PaymentMethodState>,
  Query(FindByIdParams { user_id, payment_method_id }): Query<FindByIdParams>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
  let result = module.state.find_payment_method_by_id(&payment_method_id, &user_id).await;

  match result {
    Ok(v) => Ok((StatusCode::OK, Json(v))),
    Err(e) => Err(ApplicationErrorWrapper(e)),
  }
}

pub async fn update_payment_method(
  Extension(module): Extension<PaymentMethodState>,
  Json(payload): Json<PaymentMethodDTO>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
  let result = module.state.update_payment_method(payload).await;
  let response = json!({
      "message": "payment method updated",
      "status code": StatusCode::OK.as_u16()
  });

  match result {
    Ok(_) => Ok((StatusCode::OK, Json(response))),
    Err(e) => Err(ApplicationErrorWrapper(e)),
  }
}

pub async fn delete_payment_method(
  Extension(module): Extension<PaymentMethodState>,
  Query(FindByIdParams { user_id, payment_method_id }): Query<FindByIdParams>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
  let result = module.state.delete_payment_method(&payment_method_id, &user_id).await;

  let response = json!({
      "message": "payment method deleted",
      "status code": StatusCode::OK.as_u16()
  });

  match result {
    Ok(()) => Ok((StatusCode::OK, Json(response))),
    Err(e) => Err(ApplicationErrorWrapper(e)),
  }
}
