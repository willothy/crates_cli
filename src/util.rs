pub mod terminal {
    use termion::color::{Fg, Reset};
    pub const RESET: Fg<Reset> = Fg(Reset);
}

pub mod crates {
    use crates_io_api::SyncClient;

    pub fn get_client() -> Result<SyncClient, String> {
        match SyncClient::new(
            &("crates_cli".to_owned() + clap::crate_authors!(" ")),
            std::time::Duration::from_millis(1000),
        ) {
            Ok(client) => Ok(client),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub mod error {
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
    use comfy_table::{
        modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, ContentArrangement::Dynamic, Table,
    };

    pub fn setup() -> Table {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(Dynamic);
        table
    }

    pub fn header(headers: &[&str]) -> Vec<Cell> {
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

    pub fn row(elements: &[&str]) -> Vec<Cell> {
        use comfy_table::Color;
        let mut cells = Vec::new();
        for cell in elements {
            cells.push(Cell::new(cell).fg(Color::White));
        }
        cells
    }
}
