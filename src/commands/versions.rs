use crate::util::{crates, loader, terminal::{CratesCliStyle, self}};
use std::thread;

/// Find crate and list its available versions
pub fn run(
    name: &str,
    max_results: usize,
    find: Option<&str>,
    oldest_first: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let search_name = name.to_owned();
    let handle = thread::spawn(move || {
        // Instantiate the client.
        let client = crates::get_client()?;
        // Return the CrateResponse, or an error.
        crates::get_crate(&client, search_name)
    });

    let found_crate = loader::load_until_join(handle, format!("Searching for {}", name))?;
    let num_versions = found_crate.versions.len();
    let mut crate_versions = found_crate.versions;

    if oldest_first {
        crate_versions.reverse();
    }
    if max_results > 0 {
        crate_versions.truncate(max_results);
    }
    if let Some(find) = find {
        crate_versions.retain(|version| version.num.contains(find));
    }

    terminal::print(
        format!(
            "\rFound {} available versions for {}, showing {}\n",
            num_versions.to_string().style_secondary(),
            found_crate.crate_data.name.style_primary(),
            crate_versions.len().to_string().style_secondary()
        )
    )?;

    terminal::print_queue(crate_versions.iter().map(|ver| {
        format!(
            "- {} ({})\n",
            ver.num.to_string().style_secondary(),
            ver.created_at.date().naive_local()
        )
    }).collect(), true)?;
    Ok(())
}
