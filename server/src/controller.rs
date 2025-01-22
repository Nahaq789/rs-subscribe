pub mod params;
pub mod payment_method_controller;
pub mod subscribe_controller;

use application::error::ApplicationError;

pub struct ApplicationErrorWrapper(ApplicationError);

impl std::fmt::Display for ApplicationErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl axum::response::IntoResponse for ApplicationErrorWrapper {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApplicationErrorWrapper(ApplicationError::PaymentMethodError(..)) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            _ => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        let res = serde_json::json!({
            "message": message,
            "status code": status.as_u16()
        });
        (status, axum::Json(res)).into_response()
    }
}
