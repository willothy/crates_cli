use clap::{arg, crate_authors, crate_description, crate_version, Command};

const TY_MSG: &str = "Thank you for using crates_cli!";

pub fn setup<'a>() -> Command<'a> {
    let crates = Command::new("crate")
        .version(crate_version!())
        .propagate_version(true)
        .author(crate_authors!(",\n"))
        .about(crate_description!())
        .subcommand_required(true)
        .after_long_help(TY_MSG)
        .arg_required_else_help(true)
        .subcommands(vec![
            Command::new("find")
                .author(crate_authors!(",\n"))
                .after_long_help(TY_MSG)
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
                .author(crate_authors!(",\n"))
                .after_long_help(TY_MSG)
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
        .bin_name("cargo")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // This arg doesn't do anything in crates_cli
        // I added it so that it'll show on the manpage if cargo-crates is run as as a freestanding binary and not a subcommand of cargo.
        .arg(
            arg!(--list)
                .required(false)
                .help("List all cargo subcommands"),
        )
        .subcommand(crates)
}
