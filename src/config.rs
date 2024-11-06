//! Configuration management for the application
//! 
//! This module handles all configuration aspects of the application, including
//! loading from environment variables and providing defaults.

use color_eyre::Result;
use std::net::SocketAddr;
use std::str::FromStr;
use tracing::error;
use crate::constants::{
    DEFAULT_PORT, 
    DEFAULT_BIND_ADDR,
    SSE_CHANNEL_CAPACITY,
    SSE_KEEPALIVE_INTERVAL,
    SSE_BUFFER_CAPACITY,
    DEFAULT_RATE_LIMIT,
    DEFAULT_RATE_LIMIT_BURST,
    DEFAULT_RATE_LIMIT_TIMEOUT
};
use crate::utils::get_env_or_default;
use leaky_bucket::RateLimiter;
use std::sync::Arc;

/// Main configuration structure for the application
/// 
/// Contains all configuration parameters that can be set via environment variables
/// or defaults from constants.
#[derive(Clone)]
pub struct Config {
    /// Port number the server will listen on
    pub port: u16,
    /// Network address to bind the server to
    pub bind_addr: String,
    /// OpenAI API key for authentication
    pub openai_api_key: String,
    /// Capacity of the SSE channel for streaming responses
    pub sse_channel_capacity: usize,
    /// Interval for SSE keepalive messages in seconds
    pub sse_keepalive_interval: u64,
    /// Initial capacity for SSE response buffer
    pub sse_buffer_capacity: usize,
    /// Rate limiter for API requests
    pub rate_limiter: Arc<RateLimiter>,
    /// Rate limit configuration values (for logging)
    pub rate_limit: usize,
    pub rate_limit_burst: usize,
    pub rate_limit_timeout: u64,
}

impl Config {
    /// Creates a new Config instance from environment variables
    /// 
    /// # Errors
    /// 
    /// Returns an error if required environment variables are missing
    pub async fn from_env() -> Result<Self> {
        // Use helper function for all config values
        let port = get_env_or_default("PORT", DEFAULT_PORT);
        let bind_addr = get_env_or_default("BIND_ADDRESS", DEFAULT_BIND_ADDR.to_string());
        let sse_channel_capacity = get_env_or_default("SSE_CHANNEL_CAPACITY", SSE_CHANNEL_CAPACITY);
        let sse_keepalive_interval = get_env_or_default("SSE_KEEPALIVE_INTERVAL", SSE_KEEPALIVE_INTERVAL);
        let sse_buffer_capacity = get_env_or_default("SSE_BUFFER_CAPACITY", SSE_BUFFER_CAPACITY);
        
        // Rate limit configuration
        let rate_limit = get_env_or_default("RATE_LIMIT", DEFAULT_RATE_LIMIT);
        let rate_limit_burst = get_env_or_default("RATE_LIMIT_BURST", DEFAULT_RATE_LIMIT_BURST);
        let rate_limit_timeout = get_env_or_default("RATE_LIMIT_TIMEOUT", DEFAULT_RATE_LIMIT_TIMEOUT);

        let rate_limiter = Arc::new(
            RateLimiter::builder()
                .initial(rate_limit_burst)
                .refill(1)
                .interval(std::time::Duration::from_millis(1000))
                .fair(true)
                .max(rate_limit_burst)
                .build()
        );

        // OpenAI API key still needs error handling as it's required
        let openai_api_key = std::env::var("OPENAI_API_KEY").map_err(|_| {
            error!("OPENAI_API_KEY environment variable is not set");
            color_eyre::eyre::eyre!("OPENAI_API_KEY not set")
        })?;

        Ok(Config {
            port,
            bind_addr,
            openai_api_key,
            sse_channel_capacity,
            sse_keepalive_interval,
            sse_buffer_capacity,
            rate_limiter,
            rate_limit,
            rate_limit_burst,
            rate_limit_timeout,
        })
    }

    /// Creates a SocketAddr from the configured port and bind address
    /// 
    /// # Errors
    /// 
    /// Returns an error if the address cannot be parsed
    pub fn socket_addr(&self) -> Result<SocketAddr> {
        SocketAddr::from_str(&format!("{}:{}", self.bind_addr, self.port))
            .map_err(|e| color_eyre::eyre::eyre!("Failed to create socket address: {}", e))
    }
} 