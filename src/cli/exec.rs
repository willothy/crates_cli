use crate::commands::*;
use clap::Command;
use std::error::Error;

/// Handles execution of commands from input args, using clap
pub fn execute(app: Command) -> Result<(), Box<dyn Error>> {
    let command = match app.get_matches().subcommand() {
        Some(("crate", subcommand)) => subcommand.clone(),
        _ => return Err("Expected 'crate'".into()),
    };
    match command.subcommand() {
        Some(("find", args)) => find::run(
            args.value_of("name").unwrap(),
            args.value_of("sort").unwrap(),
            if args.is_present("all") {
                None
            } else {
                Some(args.value_of_t::<usize>("max_results").unwrap_or(3))
            },
            args.is_present("filter"),
        ),
        Some(("show", args)) => show::run(match args.value_of("name") {
            Some(name) => name,
            None => return Err("No name given".into()),
        }),
        Some(("versions", args)) => versions::run(
            match args.value_of("name") {
                Some(name) => name,
                None => return Err("No name given".into()),
            },
            args.value_of_t::<usize>("max_results").unwrap_or(0),
            args.value_of("filter"),
            args.is_present("oldest_first"),
        ),
        Some(("deps", args)) => deps::run(match args.value_of("name") {
            Some(name) => name,
            None => return Err("No name given".into()),
        }),
        Some((unknown_cmd, _)) => Err(format!("Unknown command: {}", unknown_cmd).into()),
        None => Err("No command specified.".into()),
    }
}
