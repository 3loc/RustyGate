//! Utility functions
//! 
//! Common utility functions used throughout the application.

/// Gets an environment variable value or returns a default
/// 
/// # Arguments
/// 
/// * `key` - The environment variable key
/// * `default` - The default value to use if the environment variable is not set
/// 
/// # Type Parameters
/// 
/// * `T` - The type to parse the environment variable into
pub fn get_env_or_default<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr,
{
    std::env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
} 