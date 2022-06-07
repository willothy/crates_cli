//! # README
//! [![rust-clippy analyze](https://github.com/willothy/crates_cli/actions/workflows/rust-clippy.yml/badge.svg?branch=main)](https://github.com/willothy/crates_cli/actions/workflows/rust-clippy.yml)
//! <br>A command line interface for finding and managing Rust crates, heavily inspired by cargo-edit.
//!
//!
//! Check the [`Usage`](https://github.com/willothy/crates_cli/wiki/Usage) page in the [wiki](https://github.com/willothy/crates_cli/wiki) for more info on how to use `crates_cli`.
//!
//! #### Features:
//! - `find` searches crates.io for a crate
//! - `show` displays information about a crate, including homepage and documentation links.
//!
//! #### Planned features:
//! - `add`, `rm`, and `version` for managing dependencies
//! - `feature` and related subcommands for managing features of dependencies
//! - `license` retrieve the license of a crate
//!
//!
//! # Usage
//!
//! #### Show
//! `cargo crate show <name>`<br>
//!
//! #### Find
//! `cargo crate find [OPTIONS] <name>`

use std::process::ExitCode;

mod cli;
mod commands;
mod util;

fn main() -> ExitCode {
    let app = cli::setup();

    match cli::execute(app) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}
