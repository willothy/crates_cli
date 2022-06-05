use clap::*;
use cli::CliExecute;
use std::process::ExitCode;

mod cli;
mod commands;
mod util;

fn main() -> ExitCode {
    let app = cli::setup()
        .bin_name("cargo")
        .version(crate_version!())
        .author(crate_authors!(",\n"))
        .about(crate_description!())
        .name(crate_name!());

    match app.execute() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}
