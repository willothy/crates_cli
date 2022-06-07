//! Table utilities
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, ContentArrangement::Dynamic, Table,
};

/// `setup()` returns a `Table` object that has been loaded with the `UTF8_FULL` preset, modified
/// with the `UTF8_ROUND_CORNERS` modifier, and set to `Dynamic` content arrangement
///
/// Returns:
///
/// A table with the preset UTF8_FULL, the modifier UTF8_ROUND_CORNERS, and the content arrangement
/// Dynamic.
pub fn setup() -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(Dynamic);
    table
}

/// `header` takes a slice of strings and returns a vector of header cells
///
/// Arguments:
///
/// * `headers`: &[&str]
///
/// Returns:
///
/// A vector of cells.
fn style_header(headers: &[&str]) -> Vec<Cell> {
    use comfy_table::{Attribute, Color};
    let mut cells = Vec::new();
    for header in headers {
        cells.push(
            Cell::new(header)
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        );
    }
    cells
}

/// It takes a slice of strings and returns a vector of row cells
///
/// Arguments:
///
/// * `elements`: &[&str]
///
/// Returns:
///
/// A vector of cells.
fn style_row(elements: &[&str]) -> Vec<Cell> {
    use comfy_table::Color;
    let mut cells = Vec::new();
    for cell in elements {
        cells.push(Cell::new(cell).fg(Color::White));
    }
    cells
}

pub fn set_header(table: &mut Table, headers: &[&str]) {
    table.set_header(style_header(headers));
}

pub fn add_rows(table: &mut Table, rows: &[&[&str]]) {
    for row in rows {
        table.add_row(style_row(*row));
    }
}

pub fn add_row(table: &mut Table, row: &[&str]) {
    table.add_row(style_row(row));
}
