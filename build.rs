use clap::*;
use flate2;

use std::{error::Error, io::Write};

#[path = "src/cli/setup.rs"]
mod cli;

#[derive(Debug)]
struct BuildError;

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Build error")
    }
}

impl std::error::Error for BuildError {}

fn gen_man(cmd: &Command, parent: Option<&str>) -> Result<(), Box<dyn Error>> {
    use std::path::PathBuf;
    let write = parent.is_some();
    let parent = parent.unwrap_or("");

    let man = clap_mangen::Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let out_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("man");
    let out_file = out_dir.join(parent.to_owned() + "-" + cmd.get_name() + ".1.gz");
    println!("out dir: {}", out_dir.to_str().unwrap_or(""));

    if write {
        use std::fs;
        if !out_dir.exists() {
            fs::create_dir_all(&out_dir)?;
        }

        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&buffer)?;
        fs::write(
            &out_file,
            encoder.finish()?,
        )?;
    }

    for subcommand in cmd.get_subcommands() {
        gen_man(subcommand, Some(cmd.get_name()))?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let c = cli::setup();
    gen_man(&c, None)?;

    Ok(())
}
