#![warn(clippy::pedantic)]

use clap::{crate_name as name, Parser};
use log::trace;

use crate::cli::Cli;
use crate::err::Exit;

pub mod cfg;
pub mod cli;
pub mod env;
pub mod err;

/// Name of this crate.
///
/// This may be used for base subdirectories.
pub const NAME: &str = name!();

fn main() -> Exit {
    // Parse args
    let mut args = Cli::parse();
    // Load config
    args.cfg.data.merge({
        match cfg::load(&args.cfg.path) {
            Ok(conf) => conf,
            Err(err) => return err.into(),
        }
    });
    // Initialize logger
    env_logger::init();
    // Log previous steps
    trace!("{args:?}");

    // TODO: Application logic
    #[cfg(feature = "todo")]
    todo!("Hello, world!");

    // Terminate normally
    Exit::Success
}
