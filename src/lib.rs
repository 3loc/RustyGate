//! RustyGate - A Rust-based API Gateway for OpenAI services
//! 
//! This crate provides a proxy server that handles authentication and streaming
//! responses for OpenAI API calls. It supports both regular JSON responses and
//! Server-Sent Events (SSE) for streaming completions.

pub mod config;
pub mod constants;
pub mod handlers;
pub mod logging;
pub mod sse;
pub mod proxy;
pub mod utils;