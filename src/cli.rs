//! Command-line interface.

use std::path::PathBuf;

use clap::{Args, Parser, ValueHint};

use crate::cfg::{self, Config};
use crate::env;

/// TODO: Describe my application.
///
/// This will be rendered when running with `--help`.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about)]
pub struct Cli {
    /// Configuration options.
    #[clap(flatten)]
    pub cfg: Settings,

    /// TODO: Add more args.
    #[cfg(feature = "todo")]
    #[clap(short, long)]
    pub todo: Todo,
}

/// Configuration options.
#[derive(Args, Debug)]
pub struct Settings {
    /// Configuration file.
    ///
    /// When options are specified in multiple locations, they will be applied
    /// with the following precedence: cli > env > file.
    #[clap(long = "conf", env = env::CFG)]
    #[clap(value_name = "PATH")]
    #[clap(default_value_os_t = cfg::path())]
    #[clap(help_heading = None)]
    #[clap(hide_default_value = std::env::var(env::CFG).is_ok())]
    #[clap(hide_env_values    = std::env::var(env::CFG).is_err())]
    #[clap(value_hint = ValueHint::FilePath)]
    pub path: PathBuf,

    /// Configuration data.
    #[clap(flatten)]
    #[clap(next_help_heading = "Settings")]
    pub data: Config,
}
