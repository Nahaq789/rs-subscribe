use application::dtos::subscribe_dto::SubscribeDto;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;

use crate::app_state::SubscribeState;

use super::{
    params::subscribe_params::{FindAllParam, FindByIdParams},
    ApplicationErrorWrapper,
};

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

pub async fn find_subscribe_all(
    Extension(module): Extension<SubscribeState>,
    Query(FindAllParam { user_id }): Query<FindAllParam>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
    let result = module.state.find_subscribe_all(&user_id).await;

    match result {
        Ok(v) => Ok((StatusCode::OK, Json(v))),
        Err(e) => Err(ApplicationErrorWrapper(e)),
    }
}

pub async fn find_subscribe_by_id(
    Extension(module): Extension<SubscribeState>,
    Query(FindByIdParams { user_id, subscribe_id }): Query<FindByIdParams>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
    let result = module.state.find_subscribe_by_id(&user_id, &subscribe_id).await;

    match result {
        Ok(v) => Ok((StatusCode::OK, Json(v))),
        Err(e) => Err(ApplicationErrorWrapper(e)),
    }
}

pub async fn update_subscribe(
    Extension(module): Extension<SubscribeState>,
    Json(payload): Json<SubscribeDto>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
    let result = module.state.update_subscribe(payload).await;
    let response = json!({
        "message": "subscribe updated",
        "status code": StatusCode::OK.as_u16()
    });

    match result {
        Ok(_) => Ok((StatusCode::OK, Json(response))),
        Err(e) => Err(ApplicationErrorWrapper(e)),
    }
}

pub async fn delete_subscribe(
    Extension(module): Extension<SubscribeState>,
    Query(FindByIdParams { user_id, subscribe_id }): Query<FindByIdParams>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
    let result = module.state.delete_subscribe(&user_id, &subscribe_id).await;
    let response = json!({
        "message": "subscribe deleted",
        "status code": StatusCode::OK.as_u16()
    });

    match result {
        Ok(_) => Ok((StatusCode::OK, Json(response))),
        Err(e) => Err(ApplicationErrorWrapper(e)),
    }
}
