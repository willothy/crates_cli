use crate::util::{crates, loader, table};
use num_format::{Locale, ToFormattedString};
use std::thread;

pub fn show(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let search_name = name.to_owned();
    let handle = thread::spawn(move || {
        // Instantiate the client.
        let client = crates::get_client()?;
        let found_crate = match client.get_crate(search_name.trim()) {
            Ok(result) => result,
            Err(e) => return Err(e.to_string()),
        };
        Ok(found_crate)
    });

    let found_crate = match loader::load_until_join(handle, format!("Searching for {}", name)) {
        Ok(found_crate) => found_crate,
        Err(_) => return Err(format!("\nSorry, couldn't find crate {} :(", name).into()),
    };

    let mut table = table::setup();
    table.set_header(table::header(&[
        &found_crate.crate_data.name,
        &("Version ".to_owned() + &found_crate.crate_data.max_version),
    ]));

    table.add_row(table::row(&[
        "Last Update",
        &found_crate.crate_data.updated_at.to_string(),
    ]));
    table.add_row(table::row(&[
        "Description",
        &found_crate
            .crate_data
            .description
            .unwrap_or_else(|| "No description provided".to_owned()),
    ]));
    table.add_row(table::row(&[
        "Homepage",
        &found_crate
            .crate_data
            .homepage
            .unwrap_or_else(|| "No homepage provided".to_owned()),
    ]));
    table.add_row(table::row(&[
        "Docs",
        &found_crate
            .crate_data
            .documentation
            .unwrap_or_else(|| "No docs available.".to_owned()),
    ]));
    table.add_row(table::row(&[
        "Downloads",
        &found_crate
            .crate_data
            .downloads
            .to_formatted_string(&Locale::en_CA),
    ]));
    println!("\n{table}");
    Ok(())
}
