//! Command-line interface.

use std::path::PathBuf;

use clap::{Parser, ValueHint};

use crate::cfg::Config;

/// TODO: Describe my application.
///
/// This will be rendered when running with `--help`.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about)]
pub struct Cli {
    /// Configuration file.
    ///
    /// When options are specified in multiple locations, they will be applied
    /// with the following precedence: cli > env > file.
    #[arg(long, value_name = "PATH")]
    #[arg(default_value_os_t = Config::path())]
    #[arg(value_hint = ValueHint::FilePath)]
    pub conf: PathBuf,

    /// Configuration data.
    #[clap(flatten)]
    #[clap(next_help_heading = None)]
    pub cfg: Config,

    // /// TODO: Add more args.
    // #[clap(short, long)]
    // #[clap(next_help_heading = None)]
    // pub todo: bool,
}
