#![warn(clippy::pedantic)]

use anyhow::Context;
use ::log::{info, trace};
use clap::{Parser, crate_name as name};

use crate::cli::{Cli, Command};
use crate::err::{Exit, Result};

pub mod cfg;
pub mod cli;
pub mod env;
pub mod err;
pub mod r#gen;
pub mod log;
pub mod man;

/// Name of this crate.
pub const NAME: &str = name!();

/// Application entry.
fn main() -> Exit {
    // Parse args
    let args = Cli::parse();
    // Set verbosity
    log::VERBOSE
        .set(args.log)
        .expect("unable to set verbosity level");

    // Execute subcommand
    let out = match args.cmd {
        Some(Command::Gen(cli)) => {
            // rugby gen
            crate::r#gen::main(*cli)
        }
        Some(Command::Man(cli)) => {
            // rugby help
            crate::man::main(*cli)
        }
        None => exec(args),
    };

    // Return exit status
    match out {
        Ok(()) => Exit::Success,
        Err(e) => Exit::Failure(e),
    }
}

fn exec(mut args: Cli) -> Result<()> {
    // Load config
    args.cfg.data.merge(cfg::load(&args.cfg.path)?);
    // Initialize logger
    crate::log::init().context("logger initialization failed")?;
    // Log previous steps
    trace!("{args:#?}");

    // TODO: Application logic
    #[cfg(feature = "todo")]
    todo!("Hello, world!");
    info!("running application logic...");

    // Terminate normally
    Ok(())
}
