use std::error::Error;

use clap::{self, Command, arg};

pub fn setup<'a>() -> Command<'a> {
    Command::new("crates_cli")
        .propagate_version(true)
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
                    .help("The name of the crate to search for.")
                )
                .arg(
                    arg!(-m --max_results <max_results>)
                    .required(false)
                    .display_order(1)
                    .help("The maximum number of results to display, max. 50.")
                    .default_value("5")
                    .validator(|v| {
                        match v.parse::<u64>() {
                            Ok(v) => {
                                if v <= 50 {
                                    Ok(())
                                } else {
                                    Err("Maximum number of results is 50.".to_owned())
                                }
                            },
                            Err(e) => Err(e.to_string()),
                        }
                    })
                )
                .arg(
                    arg!(-s --sort <sort>)
                    .required(false)
                    .display_order(2)
                    .help("The sort order of the results.")
                    .possible_values(&["relevance", "downloads", "newly-added", "recent-downloads", "recently-updated"])
                    .default_value("relevance")
                )
        ])
}

pub trait CliExecute<'a> {
    fn execute(self) -> Result<(), Box<dyn Error>>;
}

impl<'a> CliExecute<'a> for Command<'a> {
    fn execute(self) -> Result<(), Box<dyn Error>> {
        match self.get_matches().subcommand() {
            Some(("find", args)) => crate::commands::search(
                args.value_of("name").unwrap(),
                args.value_of("sort").unwrap(),
                Some(args.value_of_t::<u64>("max_results")?),
            ),
            Some((_, _)) => {
                panic!(); // TODO: Handle invalid input
            }
            None => {
                panic!(); // TODO: Handle no input
            }
        }
    }
}