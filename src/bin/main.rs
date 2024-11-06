//! RustyGate server binary
//! 
//! The main entry point for the RustyGate server application.

use rustygate::{
    config,
    handlers,
    logging,
};

use axum::{routing::{get, post}, Router};
use color_eyre::Result;
use tower_http::trace::TraceLayer;
use tracing::info;

/// Application entry point
/// 
/// Sets up the server with all routes and middleware, then starts listening
/// for requests. Uses sane defaults for a machine with 2 CPU cores and 8GB RAM:
/// - Rate limit: 20 req/s (allows good throughput while preventing overload)
/// - Burst: 40 (2x rate limit for handling spikes)
/// - SSE buffer: 512 bytes (sufficient for typical OpenAI streaming chunks)
/// - SSE channel capacity: 1000 (balanced memory usage)
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install()?;

    // Load configuration first - now async
    let config = config::Config::from_env().await?;

    // Initialize logging with config
    logging::setup(&config)?;

    info!("Starting RustyGate with configuration:");
    info!("  Bind Address: {}:{}", config.bind_addr, config.port);
    info!("  SSE Channel Capacity: {}", config.sse_channel_capacity);
    info!("  SSE Keepalive Interval: {}s", config.sse_keepalive_interval);
    info!("  SSE Buffer Capacity: {} bytes", config.sse_buffer_capacity);
    info!("  Rate Limit: {} requests/second", config.rate_limit);
    info!("  Rate Limit Burst: {} requests", config.rate_limit_burst);

    let addr = config.socket_addr()?;

    // Create router and define routes
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/v1/*path", post(handlers::proxy_handler))
        .with_state(config)
        .layer(TraceLayer::new_for_http());

    info!("Server listening on http://{}", addr);

    // Start the server
    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app.into_make_service(),
    )
    .await?;

    Ok(())
} 