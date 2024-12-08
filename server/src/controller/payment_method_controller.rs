use crate::app_state::PaymentMethodState;
use application::error::ApplicationError;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use std::fmt;
use std::fmt::Formatter;

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
  // Json(payload): Json<PaymentMethodDTO>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
  let user_id = "usr_514ac014-fa95-0bae-437e-e5998be7be3c";
  let result = module.state.find_payment_method_all(user_id).await;

  match result {
    Ok(v) => Ok((StatusCode::OK, Json(v))),
    Err(e) => Err(ApplicationErrorWrapper(e)),
  }
}

pub async fn find_payment_method_all(
  Extension(module): Extension<PaymentMethodState>,
  Path(user_id): Path<String>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
  let result = module.state.find_payment_method_all(&user_id).await;

  match result {
    Ok(v) => Ok((StatusCode::OK, Json(v))),
    Err(e) => Err(ApplicationErrorWrapper(e)),
  }
}
