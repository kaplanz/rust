#![warn(clippy::pedantic)]

use clap::{crate_name as name, Parser};
use log::trace;

use crate::cfg::Config;
use crate::cli::Cli;
use crate::err::Exit;

pub mod cfg;
pub mod cli;
pub mod err;

/// Name of this crate.
///
/// This may be used for base subdirectories.
pub const NAME: &'static str = name!();

fn main() -> Exit {
    // Parse args
    let mut args = Cli::parse();
    // Load config
    args.cfg.merge({
        match Config::load(&args.conf) {
            Ok(conf) => conf,
            Err(err) => return err.into(),
        }
    });
    // Initialize logger
    env_logger::init();
    // Log previous steps
    trace!("{args:?}");

    // TODO: Application logic
    advise::info!("Hello, world!");

    // Terminate normally
    Exit::Success
}
