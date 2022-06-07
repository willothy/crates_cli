pub mod terminal {
    //! Terminal utilities
    use termion::color::{Fg, LightCyan, Reset};
    use termion::style::Bold;
    pub const RESET: Fg<Reset> = Fg(Reset);
    pub const PRIMARY: Fg<LightCyan> = Fg(LightCyan);
    pub const BOLD: Bold = Bold;
    pub const DEC_RESET: termion::style::Reset = termion::style::Reset;
}

pub mod crates {
    //! Crates.io API utilities
    use crates_io_api::{CrateResponse, SyncClient};

    pub fn get_client() -> Result<SyncClient, String> {
        match SyncClient::new(
            &("crates_cli".to_owned() + clap::crate_authors!(" ")),
            std::time::Duration::from_millis(1000),
        ) {
            Ok(client) => Ok(client),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn get_crate(client: &SyncClient, search_name: String) -> Result<CrateResponse, String> {
        match client.get_crate(search_name.trim()) {
            Ok(result) => Ok(result),
            Err(_) => Err(format!("Sorry, could not find crate {} :(", search_name)),
        }
    }
}

pub mod error {
    //! Error utilities
    use std::{
        error::Error,
        sync::{LockResult, MutexGuard},
    };

    pub trait NotPoison<'a, T> {
        fn not_poison(self) -> Result<MutexGuard<'a, Vec<T>>, Box<dyn Error>>;
    }

    impl<'a, T> NotPoison<'a, T> for LockResult<MutexGuard<'a, Vec<T>>> {
        fn not_poison(self) -> Result<MutexGuard<'a, Vec<T>>, Box<dyn Error>> {
            match self {
                Ok(v) => Ok(v),
                Err(e) => Err(e.to_string().into()),
            }
        }
    }
}

pub mod loader {
    //! Loading utilities
    use std::thread::JoinHandle;

    use spinners::{Spinner, Spinners};

    pub fn load_until_join<T>(
        handle: JoinHandle<Result<T, String>>,
        message: String,
    ) -> Result<T, String> {
        let mut sp = Spinner::new(Spinners::Line, message);
        match handle.join() {
            Ok(result) => match result {
                Ok(result) => {
                    sp.stop();
                    Ok(result)
                }
                Err(e) => Err(e),
            },
            Err(_) => Err("Network thread could not rejoin".to_owned()),
        }
    }
}

pub mod table {
    //! Table utilities
    use comfy_table::{
        modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, ContentArrangement::Dynamic, Table,
    };

    /// `setup()` returns a `Table` object that has been loaded with the `UTF8_FULL` preset, modified
    /// with the `UTF8_ROUND_CORNERS` modifier, and set to `Dynamic` content arrangement
    ///
    /// Returns:
    ///
    /// A table with the preset UTF8_FULL, the modifier UTF8_ROUND_CORNERS, and the content arrangement
    /// Dynamic.
    pub fn setup() -> Table {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(Dynamic);
        table
    }

    /// `header` takes a slice of strings and returns a vector of header cells
    ///
    /// Arguments:
    ///
    /// * `headers`: &[&str]
    ///
    /// Returns:
    ///
    /// A vector of cells.
    fn style_header(headers: &[&str]) -> Vec<Cell> {
        use comfy_table::{Attribute, Color};
        let mut cells = Vec::new();
        for header in headers {
            cells.push(
                Cell::new(header)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Cyan),
            );
        }
        cells
    }

    /// It takes a slice of strings and returns a vector of row cells
    ///
    /// Arguments:
    ///
    /// * `elements`: &[&str]
    ///
    /// Returns:
    ///
    /// A vector of cells.
    fn style_row(elements: &[&str]) -> Vec<Cell> {
        use comfy_table::Color;
        let mut cells = Vec::new();
        for cell in elements {
            cells.push(Cell::new(cell).fg(Color::White));
        }
        cells
    }

    pub fn set_header(table: &mut Table, headers: &[&str]) {
        table.set_header(style_header(headers));
    }

    pub fn add_rows(table: &mut Table, rows: &[&[&str]]) {
        for row in rows {
            table.add_row(style_row(*row));
        }
    }

    pub fn add_row(table: &mut Table, row: &[&str]) {
        table.add_row(style_row(row));
    }
}
