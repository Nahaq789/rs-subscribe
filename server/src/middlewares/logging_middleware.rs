use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
    middleware::Next,
};

use crate::middlewares::process_response;

pub fn logging_middleware(
    req: Request<Body>,
    next: Next,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response<Body>, StatusCode>> + Send>> {
    let method = req.method().as_str();
    let path = req.uri().path();

    tracing::info!("Started Request: {}, {}", method, path);

    let body = req.body();
    tracing::info!("Request Body: {:?}", body);

    let response = Box::pin(async move {
        let r = next.run(req).await;
        let (parts, body) = r.into_parts();

        let (bytes, res_json) = process_response(body).await.unwrap_or_default();

        //tracing::info!("res_json: {}", res_json);
        tracing::info!(
            error.message = res_json["message"].as_str().unwrap_or_default(),
            error.status_code = res_json["status code"].as_i64().unwrap_or_default(),
            "Error response received"
        );
        Ok(Response::from_parts(parts, Body::from(bytes)))
    });
    response
}
