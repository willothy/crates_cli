//! Author: [willothy](https://github.com/willothy)
//! Date: 6/6/2022
//! Build script for manpage generation

use clap::*;
use flate2::{write::GzEncoder, Compression};

use std::{
    error::Error,
    fs::{create_dir_all, write},
    io::Write,
    path::PathBuf,
};

#[path = "src/cli/setup.rs"]
mod cli;

/// Recursive manpage generation function
/// Called with None in parent to ignore the "cargo" command and only generate manpages for subcommands present in the crate
fn gen_man(cmd: &Command, parent: Option<&str>) -> Result<(), Box<dyn Error>> {
    let write_file = parent.is_some();
    let parent = parent.unwrap_or("");

    let man = clap_mangen::Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    if write_file {
        let mut out_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("man");
        if !out_dir.exists() {
            create_dir_all(&out_dir)?;
            out_dir = out_dir.canonicalize()?;
        }

        let out_file = out_dir.join(parent.to_owned() + "-" + cmd.get_name() + ".1.gz");

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&buffer)?;
        write(&out_file, encoder.finish()?)?;
    }

    for subcmd in cmd.get_subcommands() {
        gen_man(subcmd, Some(cmd.get_name()))?;
    }

    Ok(())
}

/// The main function of the build script
/// Sets up the clap CLI app, then calls gen_man to recursively generate manpages for subcommands
fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::setup();
    gen_man(&cli, None)?;

    Ok(())
}
