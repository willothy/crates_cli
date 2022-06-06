use clap::*;
use std::error::Error;
#[path = "src/cli/setup.rs"] mod cli;

fn gen_man(cmd: &Command, parent: Option<&str>) -> Result<(), Box<dyn Error>> {
    let write = parent.is_some();
    let parent = parent.unwrap_or("");

    let man = clap_mangen::Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let out_dir = std::path::PathBuf::from(std::env::current_dir()?.join("man"));

    if write {
        std::fs::write(
            out_dir.join(parent.to_owned() + "-" + cmd.get_name() + ".1"),
            buffer,
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
