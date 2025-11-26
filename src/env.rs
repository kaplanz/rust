//! Environment variables.

/// Environment prefix.
const PREFIX: &str = "APP";

/// Configuration file.
pub const CFG: &str = constcat::concat!(PREFIX, "_CFG");

/// Logging level.
pub const LOG: &str = constcat::concat!(PREFIX, "_LOG");
