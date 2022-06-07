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
