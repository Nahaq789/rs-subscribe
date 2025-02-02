use application::dtos::category_dto::CategoryDto;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;

use crate::app_state::CategoryState;

use super::{
    params::category_params::{FindAllParam, FindByIdParams},
    ApplicationErrorWrapper,
};

pub async fn create_category(
    Extension(module): Extension<CategoryState>,
    Json(payload): Json<CategoryDto>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
    let result = module.state.create_category(payload).await;
    let response = json!({
        "message": "category created",
        "status code": StatusCode::OK.as_u16()
    });

    match result {
        Ok(_) => Ok((StatusCode::OK, Json(response))),
        Err(e) => Err(ApplicationErrorWrapper(e)),
    }
}

pub async fn find_category_all(
    Extension(module): Extension<CategoryState>,
    Query(FindAllParam { user_id }): Query<FindAllParam>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
    let result = module.state.find_category_all(&user_id).await;

    match result {
        Ok(v) => Ok((StatusCode::OK, Json(v))),
        Err(e) => Err(ApplicationErrorWrapper(e)),
    }
}

pub async fn find_category_by_id(
    Extension(module): Extension<CategoryState>,
    Query(FindByIdParams { user_id, category_id }): Query<FindByIdParams>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
    let result = module.state.find_category_by_id(&user_id, &category_id).await;

    match result {
        Ok(v) => Ok((StatusCode::OK, Json(v))),
        Err(e) => Err(ApplicationErrorWrapper(e)),
    }
}

pub async fn update_category(
    Extension(module): Extension<CategoryState>,
    Json(payload): Json<CategoryDto>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
    let result = module.state.update_category(payload).await;
    let response = json!({
        "message": "category updated",
        "status code": StatusCode::OK.as_u16()
    });

    match result {
        Ok(_) => Ok((StatusCode::OK, Json(response))),
        Err(e) => Err(ApplicationErrorWrapper(e)),
    }
}

pub async fn delete_category(
    Extension(module): Extension<CategoryState>,
    Query(FindByIdParams { user_id, category_id }): Query<FindByIdParams>,
) -> Result<impl IntoResponse, ApplicationErrorWrapper> {
    let result = module.state.delete_category(&user_id, &category_id).await;
    let response = json!({
        "message": "category deleted",
        "status code": StatusCode::OK.as_u16()
    });

    match result {
        Ok(_) => Ok((StatusCode::OK, Json(response))),
        Err(e) => Err(ApplicationErrorWrapper(e)),
    }
}
