//! Minimal public-surface crate consumed by Tenbin's public-facing services.
//!
//! This crate exists to be a shared dependency between Tenbin's internal
//! monorepo and its public services (currently the verification service).
//! Anything that doesn't need to cross that boundary stays internal.

/// Hidden Road API client and the nested request/response types it uses.
pub mod hidden_road;

/// Wire-format types for broker-facing traffic. Currently only the Hidden
/// Road slice; more brokers will land here when added.
pub mod types;

/// OpenTelemetry / `tracing` setup helpers. Optional Sentry integration is
/// behind the `sentry` feature flag.
pub mod tracing;
pub use tracing::{SentryGuard, TracingConfig, init_tracing, init_tracing_with_config};
