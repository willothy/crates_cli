use crate::util::{crates, loader, table};
use num_format::{Locale, ToFormattedString};
use std::thread;

/// Find crate and show details about it in a table
pub fn run(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let search_name = name.to_owned();
    let handle = thread::spawn(move || {
        // Instantiate the client.
        let client = crates::get_client()?;
        // Return the CrateResponse, or an error.
        crates::get_crate(&client, search_name)
    });

    let found_crate = loader::load_until_join(handle, format!("Searching for {}", name))?;

    let mut table = table::setup();
    table::set_header(
        &mut table,
        &[
            &found_crate.crate_data.name,
            &("Version ".to_owned() + &found_crate.crate_data.max_version),
        ],
    );

    table::add_rows(
        &mut table,
        &[
            &[
                "Last Update",
                &found_crate.crate_data.updated_at.to_string(),
            ],
            &[
                "Description",
                &found_crate
                    .crate_data
                    .description
                    .unwrap_or_else(|| "No description provided".to_owned()),
            ],
            &[
                "Homepage",
                &found_crate
                    .crate_data
                    .homepage
                    .unwrap_or_else(|| "No homepage provided".to_owned()),
            ],
            &[
                "Docs",
                &found_crate
                    .crate_data
                    .documentation
                    .unwrap_or_else(|| "No docs available.".to_owned()),
            ],
            &[
                "Downloads",
                &found_crate
                    .crate_data
                    .downloads
                    .to_formatted_string(&Locale::en_CA),
            ],
        ],
    );

    println!("\r{table}");
    Ok(())
}
