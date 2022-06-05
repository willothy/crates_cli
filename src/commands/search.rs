use comfy_table::{
    modifiers,
    ContentArrangement,
    ColumnConstraint,
    Width,
    presets::UTF8_FULL,
    Table
};
use crates_io_api::{CratesQuery, SyncClient, Sort};
use spinners::{Spinner, Spinners};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::util::{
    table,
    terminal::RESET,
};

pub fn get_sort_type(sort: &str) -> Sort {
    match sort.trim() {
        "relevance" => Sort::Relevance,
        "downloads" => Sort::Downloads,
        "newly-added" => Sort::NewlyAdded,
        "recent-downloads" => Sort::RecentDownloads,
        "recently-updated" => Sort::RecentUpdates,
        "alphabetical" => Sort::Alphabetical,
        _ => Sort::RecentDownloads,
    }
}

pub fn search(
    name: &str,
    sort: &str,
    page_size: Option<usize>,
    filter: bool
) -> Result<(), Box<dyn std::error::Error>> {
    use termion::color::{Fg, Cyan};
    let name = Arc::new(Mutex::new(name.to_owned()));
    let sort = get_sort_type(sort);

    let found_crates = Arc::new(Mutex::new(Vec::new()));

    let thread_crates = found_crates.clone();
    let thread_name = name.clone();
    let handle = thread::spawn(move || {
        // Instantiate the client.
        let client = SyncClient::new(
            "my-user-agent (my-contact@domain.com)",
            std::time::Duration::from_millis(1000),
        )
        .unwrap();

        let query = CratesQuery::builder()
            .search(thread_name.lock().unwrap().as_str())
            .sort(sort)
            .build();

        let result = client.crates(query).unwrap();
        for crate_info in result.crates.iter() {
            thread_crates.lock().unwrap().push(crate_info.clone());
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
    let mut found_crates = found_crates.lock().unwrap();
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(modifiers::UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(table::header(&["Name", "Version", "Description", "Docs"]));

    found_crates.sort_by(|_, b| b.name.cmp(&name.lock().unwrap()));
    let showing_crates: Vec<&crates_io_api::Crate> = found_crates
    .iter()
    .filter(|x| {
        if filter {
            x.name.contains(name.lock().unwrap().as_str())
        } else {
            true
        }
    })
    .take(page_size.unwrap_or(found_crates.len()))
    .collect();
    showing_crates
    .iter()
    .for_each(|crate_info| {
        let crate_info = crate_info.to_owned();
        let mut description = crate_info
            .description
            .clone()
            .unwrap_or_else(|| "No description available".to_owned());
        if description.len() > 45 {
            description = description.chars().take(42).collect::<String>() + "..."
        }
        table.add_row(
            table::row(&[
                &crate_info.name,
                &crate_info.max_version,
                &description,
                &crate_info
                    .documentation
                    .clone()
                    .unwrap_or_else(|| "No docs available".to_owned())
            ])
        );
    });

    table.column_mut(2).unwrap().set_constraint(
        ColumnConstraint::Absolute(Width::Fixed(25))
    );
    println!("... Found {0}{1}{RESET}, showing {0}{2}{RESET}", Fg(Cyan), found_crates.len(), showing_crates.len());
    println!("{}", if table.row_iter().count() > 0 {
        table.to_string()
    } else {
        format!("Sorry, couln't find any crates named {}", name.lock().unwrap())
    });
    Ok(())
}
