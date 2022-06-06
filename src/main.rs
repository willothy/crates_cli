

use std::process::ExitCode;

mod cli;
mod commands;
mod util;

/// Creates a new `App` instance, configures it, and then executes it
///
/// Returns:
///
/// The exit code of the program.
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
