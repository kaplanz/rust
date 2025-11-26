use std::sync::OnceLock;

use anyhow::{Context, Result};
use clap_verbosity_flag::Verbosity;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

/// Default logging level.
pub const DEFAULT: log::LevelFilter = log::LevelFilter::Warn;

/// Logger verbosity flag.
pub static VERBOSE: OnceLock<Verbosity> = OnceLock::new();

/// Initializes the global logger.
///
/// # Note
///
/// Afterwards, the global logger's reload handle can be accessed via
/// [`RELOAD`].
pub fn init() -> Result<()> {
    // Extract verbosity flag
    let vlevel = VERBOSE.get().context("missing verbosity level")?;
    // Build and configure an environment filter
    let filter = EnvFilter::builder()
        .with_env_var(crate::env::LOG)
        .with_default_directive(vlevel.tracing_level_filter().into())
        .from_env()
        .context("error parsing logging filter")?;
    // Check if colors enabled
    let pretty = supports_color::on(supports_color::Stream::Stdout).is_some();
    // Install global logger
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(pretty)
                .with_filter(filter),
        )
        .try_init()
        .context("error installing logger")
}
