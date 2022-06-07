# crates_cli

[![rust-clippy analyze](https://github.com/willothy/crates_cli/actions/workflows/rust-clippy.yml/badge.svg?branch=main)](https://github.com/willothy/crates_cli/actions/workflows/rust-clippy.yml)
<br>A command line interface for finding and managing Rust crates, heavily inspired by cargo-edit.


Installation:

    cargo install crates_cli

<br>
Basic Usage:

`cargo crate [SUBCOMMAND] [OPTIONS] <crate>`
<br>

Check the [`Usage`](https://github.com/willothy/crates_cli/wiki/Usage) page in the wiki for more info on how to use `crates_cli`.

### Features:
- `find` searches crates.io for a crate
- `show` displays information about a crate, including homepage and documentation links.
- `deps` displays the dependencies of a crate.
- `versions` shows available versions for a crate, and allows you to filter

### Planned features:
- `add`, `rm`, and `version` for managing dependencies
- `feature` and related subcommands for managing features of dependencies
- `license` to retrieve the license of a crate


### Collaboration:
Collaboration is welcome. Feel free to submit issues and pull requests!

### Formatting
This project uses `rustfmt` for formatting, and `clippy` for linting.
