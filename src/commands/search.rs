use crates_io_api::{SyncClient, CratesQuery};
use crate::util::get_sort_type;

use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::Duration;

use spinners::{Spinner, Spinners};

pub fn search(name: &str, sort: &str, page_size: Option<u64>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.to_owned();
    let sort = get_sort_type(sort);

    let found_crates = Arc::new(Mutex::new(Vec::new()));

    let thread_crates = found_crates.clone();
    let handle = thread::spawn(move || {
        // Instantiate the client.
        let client = SyncClient::new(
            "my-user-agent (my-contact@domain.com)",
            std::time::Duration::from_millis(1000),
        ).unwrap();

        let query = CratesQuery::builder()
            .search(name)
            .sort(sort)
            .page_size(
                    match page_size {
                        Some(page_size) => page_size,
                        None => 4,
                    }
            )
            .build();
        let result = client.crates(query).unwrap();
        for crate_info in result.crates.iter() {
            thread_crates.lock().unwrap().push(crate_info.clone());
            //println!("{}", crate_info.name);
        }
    });
    let mut sp = Spinner::new(Spinners::Line, "Searching...".to_owned());
    loop {
        if handle.is_finished() {
            sp.stop();
            println!(""); // New line
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    handle.join().unwrap();
    found_crates.lock().unwrap().iter().for_each(|crate_info| {
        println!("found {}", crate_info.name);
    });
    Ok(())
}