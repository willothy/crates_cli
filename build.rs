use clap::*;
use flate2::{write::GzEncoder, Compression};

use std::{
    path::PathBuf,
    error::Error,
    io::Write,
    fs::{write, create_dir_all}
};

#[path = "src/cli/setup.rs"]
mod cli;

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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::setup();
    gen_man(&cli, None)?;

    Ok(())
}
