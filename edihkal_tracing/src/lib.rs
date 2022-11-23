#[cfg(feature = "test-helpers")]
pub mod test_helpers;

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

/// Configure and initialize tracing.
///
/// This sets a global default subscriber with an `env_filter` that may be configured via `RUST_LOG`.
/// Output is formatted in a bunyan structure.
///
/// # Call this only once!
/// This initializes the global default subscriber as well as `log-tracer`, so this function must
/// only be called once.
pub fn configure_tracing<Sink>(name: &str, env_filter: &str, sink: Sink)
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name.into(), sink);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set tracing subscriber");
}
