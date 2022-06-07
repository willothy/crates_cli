//! # Crates CLI
//! [![rust-clippy analyze](https://github.com/willothy/crates_cli/actions/workflows/rust-clippy.yml/badge.svg?branch=main)](https://github.com/willothy/crates_cli/actions/workflows/rust-clippy.yml)
//! <br>
//! A command line interface for finding and managing Rust crates, heavily inspired by cargo-edit.
//!
//! # About
//!
//! #### Features:
//!
//! - `find` searches crates.io for a crate
//! - `show` displays information about a crate, including homepage and documentation links.
//! - `deps` displays the dependencies of a crate.
//! - `versions` shows available versions for a crate, and allows you to filter
//!
//! #### Planned features:
//! - `add`, `rm`, and `version` for managing dependencies
//! - `feature` and related subcommands for managing features of dependencies
//! - `license` retrieve the license of a crate
//!
//! ***
//!
//! # Usage
//!
//! ### show
//! Display metadata for a crate<br>
//! Usage: `cargo crate show <name>`<br>
//!
//! ###### Args
//! `<name>`
//! <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
//! The name of a crate to view
//! <br>
//! ***
//!
//! ### Find
//! Search for a crate on crates.io<br>
//! Usage: `cargo crate find [OPTIONS] <name>`
//!
//! ###### Args
//! `<name>`
//! <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
//! The name (or partial name) of a crate to search for
//!
//! ###### Options
//! `-f, --filter`
//! <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
//! Only show packages whose names include the search term
//!
//! `-a, --all`
//! <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
//! Show all query results
//!
//! `-s, --sort`
//! <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
//! Query sort method, defaults to sorting by recent downloads
//!
//! `-m, --max_results <max_results>`
//! <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
//! Set the maximum results to show, default 3
//!
//! Note:
//! `--max_results` and `--all` are mutually exclusive. They are executed before `--filter`. This allows chaining of options, so `cargo crate find -af serde` will show all results of the query for `serde`, and `cargo crate find -fm 3 serde` will only show the first 3 results.
//!
//! ***
//! ### Versions
//! Display available versions for a crate<br>
//! Usage: `cargo crate versions [OPTIONS] <name>`
//!
//! ###### Args
//! `<name>`
//! <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
//! The name of a crate
//!
//! ###### Options
//! `-f, --filter <version>`
//! <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
//! The version (or partial version) to search for
//!
//! `-o, --oldest_first`
//! <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
//! Sort by oldest first, instead of newest first
//!
//! `-m, --max_results <max_results>`
//! <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
//! Set the maximum results to show, defaults to all
//!
//! ***
//!
//! ### Deps
//! Display dependencies of a crate<br>
//! Usage: `cargo crate show <name>`<br>
//!
//! ###### Args
//! `<name>`
//! <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
//! The name of a crate to view the dependencies of
//! <br>
//!
//! ***
//!

use std::process::ExitCode;

use crate::util::terminal;

mod cli;
mod commands;
mod util;

fn main() -> ExitCode {
    let app = cli::setup();
    terminal::set_title("Crates CLI");

    match cli::execute(app) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            terminal::print_error(format!("\n{}\n\n", e)).unwrap_or(());
            ExitCode::FAILURE
        }
    }
}
