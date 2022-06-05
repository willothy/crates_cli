use std::error::Error;
use cli::CliExecute;

mod util;
mod cli;
mod commands;

fn main() -> Result<(), Box<dyn Error>> {
    let app =cli::setup()
        .version(env!("CARGO_PKG_VERSION"))
        .author("Will Hopkins <willothyh@gmail.com>")
        .about("A command-line utility for finding crates on crates.io.");

    app.execute()
}
