//! Tracing and OpenTelemetry functionality

use opentelemetry::{KeyValue, trace::TracerProvider};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{Resource, trace::SdkTracerProvider};
use tracing_subscriber::{
    EnvFilter, layer::SubscriberExt, registry::Registry, util::SubscriberInitExt,
};

/// Configuration for tracing initialization.
#[derive(Default)]
pub struct TracingConfig {
    /// OpenTelemetry OTLP endpoint (e.g., "http://localhost:4317")
    pub otlp_endpoint: Option<String>,
    /// Service name for telemetry
    pub service_name: String,
    /// Sentry DSN for error reporting (requires `sentry` feature)
    pub sentry_dsn: Option<String>,
    /// Environment name for Sentry (e.g., "production", "staging")
    pub environment: Option<String>,
    /// Release version for Sentry (defaults to CARGO_PKG_VERSION)
    pub release: Option<String>,
}

impl TracingConfig {
    /// Create a new TracingConfig with the given service name.
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            ..Default::default()
        }
    }

    /// Set the OTLP endpoint.
    pub fn with_otlp(mut self, endpoint: impl Into<String>) -> Self {
        self.otlp_endpoint = Some(endpoint.into());
        self
    }

    /// Set the Sentry DSN.
    pub fn with_sentry(mut self, dsn: impl Into<String>) -> Self {
        self.sentry_dsn = Some(dsn.into());
        self
    }

    /// Set the environment name.
    pub fn with_environment(mut self, env: impl Into<String>) -> Self {
        self.environment = Some(env.into());
        self
    }

    /// Set the release version.
    pub fn with_release(mut self, release: impl Into<String>) -> Self {
        self.release = Some(release.into());
        self
    }
}

/// Sentry guard that must be held for the lifetime of the application.
#[cfg(feature = "sentry")]
pub struct SentryGuard {
    _guard: Option<sentry::ClientInitGuard>,
}

#[cfg(not(feature = "sentry"))]
pub struct SentryGuard;

/// Initialize tracing with full configuration. Returns a guard that must be held.
pub fn init_tracing_with_config(config: TracingConfig) -> SentryGuard {
    #[cfg(feature = "sentry")]
    let sentry_guard = config.sentry_dsn.as_ref().map(|dsn| {
        sentry::init((
            dsn.as_str(),
            sentry::ClientOptions {
                release: sentry::release_name!(),
                environment: config.environment.clone().map(std::borrow::Cow::Owned),
                attach_stacktrace: true,
                send_default_pii: true,
                ..Default::default()
            },
        ))
    });

    match (&config.otlp_endpoint, &config.sentry_dsn) {
        (Some(endpoint), _) => {
            init_otlp_tracing_internal(
                endpoint.clone(),
                config.service_name.clone(),
                config.sentry_dsn.is_some(),
            );
        }
        (None, _) => {
            init_fmt_tracing_internal(config.sentry_dsn.is_some());
        }
    }

    #[cfg(feature = "sentry")]
    {
        SentryGuard {
            _guard: sentry_guard,
        }
    }

    #[cfg(not(feature = "sentry"))]
    SentryGuard
}

/// Initialize tracing. If endpoint information is provided, uses OTLP in addition
/// to standard logging. For Sentry support, use `init_tracing_with_config`.
pub fn init_tracing<T: ToString, U: ToString>(otlp_endpoint: Option<T>, service_name: U) {
    match otlp_endpoint {
        Some(endpoint) => {
            init_otlp_tracing_internal(endpoint.to_string(), service_name.to_string(), false)
        }
        None => init_fmt_tracing_internal(false),
    }
}

/// Init tracing without OpenTelemetry; just log to stdout
fn init_fmt_tracing_internal(#[allow(unused)] with_sentry: bool) {
    let filter = EnvFilter::from_default_env();
    let fmt_layer = tracing_subscriber::fmt::layer();

    #[cfg(feature = "sentry")]
    if with_sentry {
        let sentry_layer = sentry_tracing::layer()
            .event_filter(|metadata| match metadata.level() {
                &tracing::Level::ERROR | &tracing::Level::WARN => {
                    sentry_tracing::EventFilter::Event
                }
                &tracing::Level::INFO => sentry_tracing::EventFilter::Breadcrumb,
                _ => sentry_tracing::EventFilter::Ignore,
            })
            .span_filter(|metadata| {
                // Capture request spans as breadcrumbs for context
                metadata.is_span() && *metadata.level() <= tracing::Level::INFO
            });

        Registry::default()
            .with(filter)
            .with(fmt_layer)
            .with(sentry_layer)
            .init();
        return;
    }

    Registry::default().with(filter).with(fmt_layer).init();
}

/// Initialize OpenTelemetry tracing over gRPC given an endpoint.
fn init_otlp_tracing_internal(
    otlp_endpoint: String,
    service_name: String,
    #[allow(unused)] with_sentry: bool,
) {
    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_endpoint)
        .build()
        .expect("Failed to build span exporter");

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(
            Resource::builder_empty()
                .with_attributes([KeyValue::new("service.name", service_name.clone())])
                .build(),
        )
        .build();

    let tracer = provider.tracer(service_name);

    let filter_layer = EnvFilter::from_default_env();
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    let fmt_layer = tracing_subscriber::fmt::layer();

    #[cfg(feature = "sentry")]
    if with_sentry {
        let sentry_layer = sentry_tracing::layer()
            .event_filter(|metadata| match metadata.level() {
                &tracing::Level::ERROR | &tracing::Level::WARN => {
                    sentry_tracing::EventFilter::Event
                }
                &tracing::Level::INFO => sentry_tracing::EventFilter::Breadcrumb,
                _ => sentry_tracing::EventFilter::Ignore,
            })
            .span_filter(|metadata| {
                // Capture request spans as breadcrumbs for context
                metadata.is_span() && *metadata.level() <= tracing::Level::INFO
            });

        Registry::default()
            .with(filter_layer)
            .with(telemetry_layer)
            .with(fmt_layer)
            .with(sentry_layer)
            .init();
        return;
    }

    Registry::default()
        .with(filter_layer)
        .with(telemetry_layer)
        .with(fmt_layer)
        .init();
}
