use crate::util::{
    crates, loader,
    terminal::{BOLD, DEC_RESET, PRIMARY, RESET},
};
use std::thread;

/// Find crate and list its available versions
pub fn versions(
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

    println!(
        "\rFound {PRIMARY}{}{RESET} available versions for {BOLD}{PRIMARY}{}{DEC_RESET}{RESET}, showing {PRIMARY}{}{RESET}",
        num_versions,
        found_crate.crate_data.name,
        crate_versions.len()
    );
    crate_versions.iter().for_each(|crate_version| {
        println!(
            "- {PRIMARY}{}{RESET} ({})",
            crate_version.num,
            crate_version.created_at.date().naive_local()
        );
    });
    Ok(())
}
