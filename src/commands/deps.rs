use crate::util::{
    crates, loader, table,
    terminal::{self, CratesCliStyle},
};
use std::thread;

/// Find crate and show details about it in a table
pub fn run(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let search_name = name.to_owned();
    let handle = thread::spawn(move || {
        // Instantiate the client.
        let client = crates::get_client()?;
        // Return the CrateResponse, or an error.
        let version = crates::get_crate(&client, search_name.clone())?
            .crate_data
            .max_version;
        match client.crate_dependencies(&search_name, &version) {
            Ok(dependencies) => Ok((dependencies, version)),
            Err(e) => Err(e.to_string()),
        }
    });

    let (crate_dependencies, version) =
        loader::load_until_join(handle, format!("Searching for {}", name))?;

    let mut table = table::setup();
    table::set_header(&mut table, &["Crate", "Version", "Required?", "Kind"]);

    crate_dependencies.iter().for_each(|dep| {
        table::add_row(
            &mut table,
            &[
                &dep.crate_id,
                &dep.version_id.to_string(),
                if dep.optional { "Optional" } else { "Required" },
                &dep.kind,
            ],
        );
    });
    terminal::print(format!(
        "\rFound {} dependencies for {} version {}\n{}\n\n",
        crate_dependencies.len().to_string().style_secondary(),
        name.style_primary(),
        version.style_secondary(),
        table
    ))?;
    Ok(())
}
