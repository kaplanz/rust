//! Show help information.

use std::process;

use anyhow::Context;
use clap::CommandFactory;
use constcat::concat;
use log::{debug, trace};

use crate::Result;
use crate::r#gen::man;

pub mod cli;

pub use self::cli::Cli;

/// Subcommand name.
pub const NAME: &str = concat!(crate::NAME, "-help");

/// [`Help`](crate::cli::Command::Help) entrypoint.
#[expect(clippy::needless_pass_by_value)]
pub fn main(args: Cli) -> Result<()> {
    // Initialize logger
    crate::log::init().context("logger initialization failed")?;
    // Log arguments
    trace!("{args:#?}");

    // Build command
    let mut cmd = match args.cmd {
        None => crate::Cli::command(),
        Some(cli::Command::Gen) => crate::r#gen::Cli::command(),
        Some(cli::Command::Man) => crate::man::Cli::command(),
    }
    .flatten_help(true);
    cmd.build();

    // Generate manpage to tempfile
    let mut tmp = tempfile::NamedTempFile::new().context("could not open temporary file")?;
    man::make(cmd, tmp.as_file_mut())?;
    debug!("wrote temporary manpage: {}", tmp.path().display());

    // Spawn manual documentation process
    debug!("launching `man`");
    let exit = process::Command::new("man")
        .arg(tmp.path())
        .status()
        .context("could not open manual page")?;
    debug!("{exit}");

    // Forward exit status
    if let Some(code) = exit.code() {
        process::exit(code);
    } else {
        Ok(())
    }
}
