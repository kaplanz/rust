//! Generate static files.

use anyhow::Context;
use constcat::concat;
use log::trace;

use crate::Result;

pub mod cli;

pub use self::cli::Cli;

/// Subcommand name.
pub const NAME: &str = concat!(crate::NAME, "-gen");

/// [`Gen`](crate::cli::Command::Gen) entrypoint.
#[expect(clippy::needless_pass_by_value)]
pub fn main(args: Cli) -> Result<()> {
    // Initialize logger
    crate::log::init().context("logger initialization failed")?;
    // Log arguments
    trace!("{args:#?}");

    // Execute subcommand
    match args.document {
        cli::Document::Cmp { shell } => cmp::exec(shell),
        cli::Document::Man { cmd } => man::exec(cmd),
    }
}

/// Shell completions.
pub mod cmp {
    use std::io::Write;

    use clap::{Command, CommandFactory};
    use clap_complete::Shell;
    use log::debug;

    use crate::{NAME, Result};

    /// [`Cmp`](super::cli::Document::Cmp) entrypoint.
    pub fn exec(shell: Shell) -> Result<()> {
        // Build command
        let mut cmd = crate::Cli::command().flatten_help(true);
        cmd.build();
        // Declare buffer
        let buf = std::io::stdout();
        // Generate output
        self::make(shell, cmd, buf)
    }

    /// Generate shell completions.
    pub fn make(shell: Shell, mut cmd: Command, mut buf: impl Write) -> Result<()> {
        debug!("generating completions: `{shell}`");
        clap_complete::generate(shell, &mut cmd, NAME, &mut buf);
        Ok(()) // unconditionally succeed
    }
}

/// Manual pages.
pub mod man {
    use std::io::Write;

    use anyhow::Context;
    use clap::{Command, CommandFactory};
    use clap_mangen::Man;
    use log::debug;

    use super::cli::Command as Subcommand;
    use crate::{NAME, Result};

    /// Manual section.
    ///
    /// The sections of the manual are:
    /// 1.  General Commands Manual
    /// 2.  System Calls Manual
    /// 3.  Library Functions Manual
    /// 4.  Kernel Interfaces Manual
    /// 5.  File Formats Manual
    /// 6.  Games Manual
    /// 7.  Miscellaneous Information Manual
    /// 8.  System Manager's Manual
    /// 9.  Kernel Developer's Manual
    pub const MANSECT: &str = "1";

    /// [`Man`](super::cli::Document::Man) entrypoint.
    pub fn exec(cmd: Option<Subcommand>) -> Result<()> {
        // Build command
        let mut cmd = match cmd {
            None => crate::Cli::command(),
            Some(Subcommand::Gen) => crate::r#gen::Cli::command(),
            Some(Subcommand::Man) => crate::man::Cli::command(),
        }
        .flatten_help(true);
        cmd.build();
        // Declare buffer
        let mut buf = std::io::stdout();
        // Generate output
        self::make(cmd, &mut buf)
    }

    /// Generate manual page.
    pub fn make(cmd: Command, mut buf: impl Write) -> Result<()> {
        debug!("generating manual page: `{cmd}`");
        Man::new(cmd)
            .title(NAME)
            .section(MANSECT)
            .render(&mut buf)
            .context("could not generate man page")
            .map_err(Into::into)
    }
}
