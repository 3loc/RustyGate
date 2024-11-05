//! HTTP request handlers
//! 
//! This module contains the main request handlers for the application,
//! including the health check endpoint and the OpenAI proxy handler.

use axum::{
    body::Body,
    extract::{Path, State},
    http::{Request, StatusCode, HeaderMap, Response},
    response::IntoResponse,
};
use bytes::Bytes;
use http_body_util::BodyExt;
use tracing::{error, info, debug};
use crate::{config::Config, sse, proxy};
use std::time::Duration;

/// Simple health check endpoint that returns "OK"
pub async fn health_check() -> &'static str {
    "OK"
}

/// Main proxy handler for OpenAI API requests
/// 
/// This handler forwards requests to OpenAI's API, handling both streaming
/// and non-streaming responses appropriately.
/// 
/// # Arguments
/// 
/// * `config` - Application configuration
/// * `path` - The API path being requested
/// * `headers` - Request headers
/// * `req` - The incoming request body
/// 
/// # Errors
/// 
/// Returns a StatusCode error if the request cannot be processed
pub async fn proxy_handler(
    State(config): State<Config>,
    Path(path): Path<String>,
    headers: HeaderMap,
    req: Request<Body>,
) -> Result<Response<Body>, StatusCode> {
    // Try to acquire rate limit token with longer timeout
    match tokio::time::timeout(
        Duration::from_secs(config.rate_limit_timeout),
        config.rate_limiter.acquire(1)
    ).await {
        Ok(_) => {
            info!("Rate limit token acquired for path: {} (waited in queue if needed)", path);
        }
        Err(_) => {
            error!("Rate limit timeout after {}s for path: {}", config.rate_limit_timeout, path);
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }

    info!("Received request for path: {}", path);
    debug!("Request headers: {:?}", headers);

    let client = reqwest::Client::new();
    let openai_url = format!("https://api.openai.com/v1/{}", path);

    let body_bytes: Bytes = req
        .into_body()
        .collect()
        .await
        .map_err(|e| {
            error!("Failed to read request body: {}", e);
            StatusCode::BAD_REQUEST
        })?
        .to_bytes();

    let mut request_builder = client
        .post(&openai_url)
        .header("Authorization", format!("Bearer {}", config.openai_api_key))
        .header("Content-Type", "application/json");

    // Forward the original Accept header
    if let Some(accept) = headers.get("accept") {
        if let Ok(accept_str) = accept.to_str() {
            request_builder = request_builder.header("Accept", accept_str);
        }
    }

    let response = request_builder
        .body(body_bytes)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to forward request to OpenAI: {}", e);
            StatusCode::BAD_GATEWAY
        })?;

    debug!("OpenAI response status: {}", response.status());
    debug!("OpenAI response headers: {:?}", response.headers());

    // Check if OpenAI is responding with a streaming response
    let is_streaming = response
        .headers()
        .get("content-type")
        .and_then(|h| h.to_str().ok())
        .map(|h| h.contains("text/event-stream"))
        .unwrap_or(false);

    if is_streaming {
        info!("Handling streaming response");
        Ok(sse::handle_sse_response(response, &config).await.into_response())
    } else {
        info!("Handling normal response");
        proxy::handle_normal_response(response).await
    }
} 