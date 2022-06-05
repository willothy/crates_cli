use clap::*;

fn main() -> std::io::Result<()> {
    let crates = Command::new("crate")
        .bin_name("")
        .version(crate_version!())
        .author(crate_authors!(",\n"))
        .subcommand_required(true)
        .after_long_help("")
        .arg_required_else_help(true)
        .subcommands(vec![
            Command::new("find")
                .after_long_help("")
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
                .after_long_help("")
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
    let cmd = Command::new("cargo")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(crates);

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let out_dir = std::path::PathBuf::from(std::env::current_dir()?);
    std::fs::write(out_dir.join("mybin.man"), buffer)?;

    Ok(())
}
