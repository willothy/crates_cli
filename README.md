# crates_cli

[![rust-clippy analyze](https://github.com/willothy/crates_cli/actions/workflows/rust-clippy.yml/badge.svg?branch=main)](https://github.com/willothy/crates_cli/actions/workflows/rust-clippy.yml)
<br>A command line interface for finding and managing Rust crates, heavily inspired by cargo-edit.


Check the [`Usage`](https://github.com/willothy/crates_cli/wiki/Usage) page in the [wiki](https://github.com/willothy/crates_cli/wiki) for more info on how to use `crates_cli`.

### Features:
- `find` searches crates.io for a crate
- `show` displays information about a crate, including homepage and documentation links.

#### Show
`cargo crate show <name>`<br><br>

#### Find
`cargo crate find [OPTIONS] <name>`
###### Options
`-f, --filter`&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
Only show packages whose names include the search term.

`-a, --all`&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
Show all query results.

`-s, --sort`&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
Query sort method - defaults to sorting by recent downloads.

`-m, --max_results <max_results>`&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
Set the maximum results to show. Default 3.

`--max_results` and `--all` are mutually exclusive. They are executed before `--filter`. This allows chaining of options, so `cargo crate find -af serde` will show all results of the query for `serde`, and `cargo crate find -fm 3 serde` will only show the first 3 results.

### Planned features:
- `add`, `rm`, and `version` for managing dependencies
- `feature` and related subcommands for managing features of dependencies
- `license` retrieve the license of a crate


### Collaboration:
Collaboration is welcome, both in the form of issues/ideas and pull requests! I'm still learning rust so feedback would be appreciated.

### Formatting
This project uses `rustfmt` for formatting, and `clippy` for linting.
