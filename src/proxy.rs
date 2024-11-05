//! HTTP proxy response handling
//! 
//! Handles non-streaming responses from OpenAI.

use axum::{
    body::Body,
    http::{StatusCode, Response},
};
use tracing::error;
use reqwest::Response as ReqwestResponse;

/// Handles a normal (non-streaming) response from OpenAI
/// 
/// # Arguments
/// 
/// * `response` - The response from OpenAI
/// 
/// # Errors
/// 
/// Returns a StatusCode error if the response cannot be processed
pub async fn handle_normal_response(response: ReqwestResponse) -> Result<Response<Body>, StatusCode> {
    let status = StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::OK);
    
    let body = response.bytes().await.map_err(|e| {
        error!("Failed to read response: {}", e);
        StatusCode::BAD_GATEWAY
    })?;

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Body::from(body))
        .map_err(|e| {
            error!("Failed to build response: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?)
} 