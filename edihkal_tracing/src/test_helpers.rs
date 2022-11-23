use once_cell::sync::Lazy;

use crate::configure_tracing;

// Ensure that `tracing` is only initialized once.
pub static TRACING: Lazy<()> = Lazy::new(|| {
    // Use TEST_LOG to determine whether test logs should output.
    if std::env::var("TEST_LOG").is_ok() {
        configure_tracing("test", "info", std::io::stdout);
    }
});

/// Lazy initialization of tracing to ensure subscriber is only initialized once.
///
/// This can be used in test helpers elsewhere for providing tracing output in tests.
///
/// Tracing output in tests is disabled by default and can be enabled using `TEST_LOG=1`.
#[allow(dead_code)]
pub fn lazy_tracing() {
    Lazy::force(&TRACING);
}
