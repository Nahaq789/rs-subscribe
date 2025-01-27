use crate::app_state::PaymentMethodState;
use application::dtos::payment_method_dto::PaymentMethodDTO;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde_json::json;

use super::params::payment_method_params::{FindAllParam, FindByIdParams};
use super::ApplicationErrorWrapper;

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
