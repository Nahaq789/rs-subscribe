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

use super::params::payment_method_params::PaymentParams;

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
    (status, message).into_response()
  }
}

pub async fn create_payment_method(
  Extension(module): Extension<PaymentMethodState>,
  Json(payload): Json<PaymentMethodDTO>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
  let result = module.state.create_payment_method(payload).await;
  let response = json!({
      "message": "payment method created",
      "status code": 200
  });

  match result {
    Ok(_) => Ok((StatusCode::OK, Json(response))),
    Err(e) => Err(ApplicationErrorWrapper(e)),
  }
}

pub async fn find_payment_method_all(
  Extension(module): Extension<PaymentMethodState>,
  Query(PaymentParams {
    user_id,
    payment_id: _,
  }): Query<PaymentParams>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
  let result = module.state.find_payment_method_all(&user_id).await;

  match result {
    Ok(v) => Ok((StatusCode::OK, Json(v))),
    Err(e) => Err(ApplicationErrorWrapper(e)),
  }
}

pub async fn find_payment_method_by_id(
  Extension(module): Extension<PaymentMethodState>,
  Query(PaymentParams {
    user_id,
    payment_id,
  }): Query<PaymentParams>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
  let result = module
    .state
    .find_payment_method_by_id(&payment_id, &user_id)
    .await;

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
      "status code": 200
  });

  match result {
    Ok(_) => Ok((StatusCode::OK, Json(response))),
    Err(e) => Err(ApplicationErrorWrapper(e)),
  }
}
