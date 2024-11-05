//! Default configuration values
//! 
//! This module contains all default values used throughout the application.

/// Default port number for the server
pub const DEFAULT_PORT: u16 = 8080;
/// Default network address to bind to
pub const DEFAULT_BIND_ADDR: &str = "127.0.0.1";
/// Default logging filter configuration
pub const DEFAULT_LOG_FILTER: &str = "rustygate=debug,tower_http=debug";

/// Default capacity for SSE channels
pub const SSE_CHANNEL_CAPACITY: usize = 100;
/// Default interval for SSE keepalive messages (in seconds)
pub const SSE_KEEPALIVE_INTERVAL: u64 = 15;
/// Default initial capacity for SSE response buffers
pub const SSE_BUFFER_CAPACITY: usize = 1024;

/// Default rate limit (requests per second)
pub const DEFAULT_RATE_LIMIT: usize = 1;  // Only 1 request per second
/// Default rate limit burst capacity (increased to allow queuing)
pub const DEFAULT_RATE_LIMIT_BURST: usize = 5;  // Smaller buffer to make rate limiting more visible
/// Default rate limit wait timeout (in seconds)
pub const DEFAULT_RATE_LIMIT_TIMEOUT: u64 = 30;  // Wait up to 30 seconds for a token