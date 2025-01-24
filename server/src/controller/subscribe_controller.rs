use application::dtos::subscribe_dto::SubscribeDto;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;

use crate::app_state::SubscribeState;

use super::ApplicationErrorWrapper;

pub async fn create_subscribe(
    Extension(module): Extension<SubscribeState>,
    Json(payload): Json<SubscribeDto>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
    let result = module.state.create_subscribe(payload).await;
    let response = json!({
     "message": "subscribe created",
     "status code": StatusCode::OK.as_u16()
    });

    match result {
        Ok(_) => Ok((StatusCode::OK, Json(response))),
        Err(e) => Err(ApplicationErrorWrapper(e)),
    }
}
