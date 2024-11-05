//! Logging configuration
//! 
//! Sets up structured logging with tracing subscriber.

use color_eyre::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use crate::config::Config;

/// Initializes the logging system with the provided configuration
/// 
/// # Arguments
/// 
/// * `config` - Application configuration containing logging settings
/// 
/// # Errors
/// 
/// Returns an error if logging setup fails
pub fn setup(_config: &Config) -> Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_file(true)
                .with_line_number(true)
                .with_ansi(true),
        )
        .init();

    Ok(())
} 