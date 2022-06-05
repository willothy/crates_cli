use crate::util::get_sort_type;

use crates_io_api::{CratesQuery, SyncClient};
use spinners::{Spinner, Spinners};
use termion::color::*;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const RESET: Fg<Reset> = Fg(Reset);

pub fn search(
    name: &str,
    sort: &str,
    page_size: Option<u64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.to_owned();
    let sort = get_sort_type(sort);

    let found_crates = Arc::new(Mutex::new(Vec::new()));

    let thread_crates = found_crates.clone();
    let handle = thread::spawn(move || {
        // Instantiate the client.
        let client = SyncClient::new(
            "my-user-agent (my-contact@domain.com)",
            std::time::Duration::from_millis(1000),
        )
        .unwrap();

        let query = CratesQuery::builder()
            .search(name)
            .sort(sort)
            .page_size(page_size.unwrap_or(4))
            .build();
        let result = client.crates(query).unwrap();
        for crate_info in result.crates.iter() {
            thread_crates.lock().unwrap().push(crate_info.clone());
            //println!("{}", crate_info.name);
        }
    });
    let mut sp = Spinner::new(Spinners::Line, "Searching".to_owned());
    loop {
        if handle.is_finished() {
            sp.stop();
            break;
        }
        thread::sleep(Duration::from_millis(150));
    }
    handle.join().unwrap();
    let found_crates = found_crates.lock().unwrap();
    println!("... Found {}{}{RESET}", Fg(LightBlue), found_crates.len());
    found_crates.iter().for_each(|crate_info| {
        println!(
            "{}- {RESET}{}: {}",
            Fg(LightBlue),
            crate_info.name,
            crate_info
                .documentation
                .clone()
                .unwrap_or_else(|| "No docs available".to_owned())
        );
    });
    Ok(())
}
