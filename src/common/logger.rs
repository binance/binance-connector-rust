use std::sync::Once;
use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt};

static INIT_LOG: Once = Once::new();

/// Initializes the global tracing subscriber with a configured log level and formatting.
///
/// This function sets up a tracing subscriber with a default log level of "info",
/// which can be overridden by environment variables. It configures the log output
/// to disable log targets, and enable thread IDs and thread names.
///
/// The initialization is performed only once using a `Once` synchronization primitive
/// to ensure thread-safe global subscriber setup.
pub fn init() {
    INIT_LOG.call_once(|| {
        if tracing::dispatcher::has_been_set() {
            return;
        }

        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        let fmt_layer = fmt::layer()
            .with_target(false)
            .with_thread_ids(true)
            .with_thread_names(true);

        let subscriber = Registry::default().with(filter).with(fmt_layer);
        let _ = tracing::subscriber::set_global_default(subscriber);
    });
}
