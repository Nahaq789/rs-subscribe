use std::usize;

use axum::body::{to_bytes, Body, Bytes};

pub mod logging_middleware;

pub fn process_response(
    body: Body,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<(Bytes, serde_json::Value), Box<dyn std::error::Error>>> + Send>,
> {
    let result = Box::pin(async move {
        let bytes = to_bytes(body, usize::MAX).await?;
        let body_string = String::from_utf8(bytes.to_vec())?;
        let res_json = serde_json::from_str(&body_string)?;

        Ok((bytes, res_json))
    });
    result
}
