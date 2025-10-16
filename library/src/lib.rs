// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![doc = include_str!("../../README.md")]

/// Configuration.
pub mod configuration;

/// File modification coordinator.
pub mod coordinator;

/// Axum middleware.
pub mod middleware;

/// Compris resolvers.
pub mod resolve;

/// Render.
pub mod render;

/// Server.
pub mod server;

/// Utilities.
pub mod util;
