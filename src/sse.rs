//! Server-Sent Events (SSE) handling
//! 
//! This module handles streaming responses from OpenAI using SSE.

use axum::response::sse::{Event, Sse};
use axum::response::IntoResponse;
use futures_util::TryStreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::{error, debug, info};
use std::{convert::Infallible, time::Duration};
use reqwest::Response;
use crate::config::Config;

/// Creates an SSE event from a data line if it starts with "data: "
/// 
/// # Arguments
/// 
/// * `line` - The line to parse as an SSE event
#[inline]
fn create_event(line: &str) -> Option<Event> {
    // Use starts_with for fast prefix checking
    if line.starts_with("data: ") {
        // Avoid allocation if the line is empty after trim
        let event_data = &line[6..];  // "data: " is 6 chars
        if !event_data.is_empty() {
            debug!("Processing event data: {}", event_data);
            Some(Event::default().data(event_data))
        } else {
            None
        }
    } else {
        None
    }
}

/// Process complete lines from the buffer, returning the remaining incomplete data
/// 
/// # Arguments
/// 
/// * `buffer` - The buffer containing SSE data
/// * `tx` - Channel sender for SSE events
async fn process_buffer(buffer: &str, tx: &mpsc::Sender<Result<Event, Infallible>>) -> String {
    let mut start = 0;
    let mut current_buffer = String::with_capacity(buffer.len());
    
    // Process lines without allocating new strings for each line
    while let Some(end) = buffer[start..].find('\n') {
        let absolute_end = start + end;
        let line = &buffer[start..absolute_end];
        start = absolute_end + 1;
        
        // Skip empty lines
        if line.is_empty() {
            continue;
        }
        
        if let Some(event) = create_event(line.trim()) {
            if let Err(e) = tx.send(Ok(event)).await {
                error!("Failed to send event: {}", e);
                // If we fail to send, preserve remaining buffer
                if start < buffer.len() {
                    current_buffer.push_str(&buffer[start..]);
                }
                return current_buffer;
            }
        }
    }
    
    // Add remaining incomplete line to buffer
    if start < buffer.len() {
        current_buffer.push_str(&buffer[start..]);
    }
    
    current_buffer
}

/// Handles an SSE response from OpenAI
/// 
/// Sets up streaming response handling with keepalive support
/// 
/// # Arguments
/// 
/// * `response` - The response from OpenAI
/// * `config` - Application configuration
pub async fn handle_sse_response(response: Response, config: &Config) -> impl IntoResponse {
    info!("Starting SSE response handler");
    debug!("SSE response headers: {:?}", response.headers());

    let (tx, rx) = mpsc::channel::<Result<Event, Infallible>>(config.sse_channel_capacity);
    
    let buffer_capacity = config.sse_buffer_capacity;
    tokio::spawn(async move {
        info!("Spawned SSE processing task");
        let mut buffer = String::with_capacity(buffer_capacity);
        
        let mut stream = response.bytes_stream();
        while let Ok(Some(chunk)) = stream.try_next().await {
            buffer.push_str(&String::from_utf8_lossy(&chunk));
            buffer = process_buffer(&buffer, &tx).await;
        }

        // Process any remaining data in the buffer
        if !buffer.is_empty() {
            if let Some(event) = create_event(buffer.trim()) {
                let _ = tx.send(Ok(event)).await;
            }
        }

        info!("SSE processing task completed");
    });

    info!("Setting up SSE response");
    Sse::new(ReceiverStream::new(rx))
        .keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(config.sse_keepalive_interval))
                .text("keep-alive")
        )
} 