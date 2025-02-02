use std::usize;

use axum::{
    body::{to_bytes, Body},
    extract::Request,
    http::{Response, StatusCode},
    middleware::Next,
};

pub fn logging_middleware(
    req: Request<Body>,
    next: Next,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response<Body>, StatusCode>> + Send>> {
    let method = req.method().as_str().to_string();
    let path = req.uri().path().to_string();

    tracing::info!("Started Request: {}, {}", method, path);

    Box::pin(async move {
        let start_time = std::time::Instant::now();
        let res = next.run(req).await;

        let status = res.status();
        tracing::info!(
            method = method,
            path = path,
            status = status.as_u16(),
            elapsed = start_time.elapsed().as_millis(),
            "Request completed"
        );

        if status.is_client_error() || status.is_server_error() {
            let (parts, body) = res.into_parts();

            if let Ok(bytes) = to_bytes(body, usize::MAX).await {
                #[cfg(debug_assertions)]
                {
                    if let Ok(body_str) = String::from_utf8(bytes.to_vec()) {
                        tracing::debug!(response_body = body_str, "Error response body")
                    };
                }
                if let Ok(error_json) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                    tracing::error!(
                        error.message = error_json["message"].as_str().unwrap_or_default(),
                        error.status_code = error_json["status_code"].as_u64().unwrap_or_default(),
                        "Error details"
                    );
                }
                Ok(Response::from_parts(parts, Body::from(bytes)))
            } else {
                Ok(Response::from_parts(parts, Body::empty()))
            }
        } else {
            Ok(res)
        }
    })
}
