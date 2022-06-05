use std::{env, error::Error};

use clap::{self, arg, crate_authors, crate_version, Command};

pub fn setup<'a>() -> Command<'a> {
    let crates = Command::new("crate")
        .version(crate_version!())
        .author(crate_authors!(",\n"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(vec![
            Command::new("find")
                .display_order(0)
                .about("Search for crates on crates.io.")
                .arg(
                    arg!([name])
                        .required(true)
                        .display_order(0)
                        .help("The name of the crate to search for."),
                )
                .arg(
                    arg!(-m --max_results <max_results>)
                        .required(false)
                        .display_order(1)
                        .help("The maximum number of results to display, max. 50.")
                        .default_value("4")
                        .validator(|v| match v.parse::<u64>() {
                            Ok(v) => {
                                if v <= 50 {
                                    Ok(())
                                } else {
                                    Err("Maximum number of results is 50.".to_owned())
                                }
                            }
                            Err(e) => Err(e.to_string()),
                        }),
                )
                .arg(
                    arg!(-s --sort <sort>)
                        .required(false)
                        .display_order(2)
                        .help("The sort order of the results.")
                        .possible_values(&[
                            "relevance",
                            "downloads",
                            "newly-added",
                            "recent-downloads",
                            "recently-updated",
                            "alphabetical",
                        ])
                        .default_value("recent-downloads"),
                )
                .arg(
                    arg!(-f - -filter)
                        .required(false)
                        .display_order(3)
                        .help("Filter out crates whose titles don't contain the search term."),
                )
                .arg(
                    arg!(-a - -all)
                        .required(false)
                        .display_order(3)
                        .help("Show all results.")
                        .conflicts_with("max_results"),
                ),
            Command::new("show")
                .display_order(0)
                .about("Display metadata about a crate.")
                .arg_required_else_help(true)
                .arg(
                    arg!([name])
                        .required(false)
                        .display_order(0)
                        .help("The name of the crate to show."),
                ),
        ]);
    Command::new("cargo")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(crates)
}

pub trait CliExecute<'a> {
    fn execute(self) -> Result<(), Box<dyn Error>>;
}

impl<'a> CliExecute<'a> for Command<'a> {
    fn execute(self) -> Result<(), Box<dyn Error>> {
        let command = match self.get_matches().subcommand() {
            Some(("crate", subcommand)) => subcommand.clone(),
            _ => return Err("Expected 'crates'".into()),
        };
        match command.subcommand() {
            Some(("find", args)) => crate::commands::find(
                args.value_of("name").unwrap(),
                args.value_of("sort").unwrap(),
                if args.is_present("all") {
                    None
                } else {
                    Some(args.value_of_t::<usize>("max_results").unwrap_or(3))
                },
                args.is_present("filter"),
            ),
            Some(("show", args)) => crate::commands::show(match args.value_of("name") {
                Some(name) => name,
                None => return Err("No name given".into()),
            }),
            Some((unknown_cmd, _)) => Err(format!("Unknown command: {}", unknown_cmd).into()),
            None => Err("No command specified.".into()),
        }
    }
}
