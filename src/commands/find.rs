use comfy_table::{
    modifiers, presets::UTF8_FULL, ColumnConstraint, ContentArrangement, Table, Width,
};
use crates_io_api::{CratesQuery, Sort};

use std::sync::{Arc, Mutex};
use std::thread;

use crate::util::error::NotPoison;
use crate::util::{crates, loader};
use crate::util::{table, terminal::RESET};

fn get_sort_type(sort: &str) -> Sort {
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

pub fn find(
    name: &str,
    sort: &str,
    page_size: Option<usize>,
    filter: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use termion::color::{Cyan, Fg};
    let name = name.to_owned();
    let sort = get_sort_type(sort);

    let found_crates = Arc::new(Mutex::new(Vec::new()));

    let search_name = name.clone();
    let handle = thread::spawn(move || {
        // Instantiate the client.
        let client = crates::get_client()?;

        let query = CratesQuery::builder()
            .search(search_name)
            .sort(sort)
            .build();

        let result = match client.crates(query) {
            Ok(result) => result,
            Err(e) => return Err(e.to_string()),
        };

        {
            let mut found_crates = match found_crates.lock() {
                Ok(found_crates) => found_crates,
                Err(e) => return Err(e.to_string()),
            };
            for crate_info in result.crates.iter() {
                found_crates.push(crate_info.clone());
            }
        }
        Ok(found_crates)
    });

    let loaded = loader::load_until_join(handle, "Searching".to_owned())?;
    let mut found_crates = loaded.lock().not_poison()?;
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(modifiers::UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(table::header(&["Name", "Version", "Description", "Docs"]));

    found_crates.sort_by(|_, b| b.name.cmp(&(*name).to_owned()));
    let showing_crates: Vec<&crates_io_api::Crate> = found_crates
        .iter()
        .filter(|x| if filter { x.name.contains(&name) } else { true })
        .take(page_size.unwrap_or(found_crates.len()))
        .collect();
    showing_crates.iter().for_each(|crate_info| {
        let mut description = crate_info
            .description
            .clone()
            .unwrap_or_else(|| "No description available".to_owned());
        if description.len() > 45 {
            description = description.chars().take(42).collect::<String>() + "..."
        }
        table.add_row(table::row(&[
            &crate_info.name,
            &crate_info.max_version,
            &description,
            &crate_info
                .documentation
                .clone()
                .unwrap_or_else(|| "No docs available".to_owned()),
        ]));
    });

    table
        .column_mut(2)
        .unwrap()
        .set_constraint(ColumnConstraint::Absolute(Width::Fixed(25)));
    println!(
        "... Found {0}{1}{RESET}, showing {0}{2}{RESET}",
        Fg(Cyan),
        found_crates.len(),
        showing_crates.len()
    );
    println!(
        "{}",
        if table.row_iter().count() > 0 {
            table.to_string()
        } else {
            format!("Sorry, couln't find any crates named {}", name)
        }
    );
    Ok(())
}
