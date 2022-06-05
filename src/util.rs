
pub mod terminal {
    use termion::color::{Fg, Reset};
    pub const RESET: Fg<Reset> = Fg(Reset);
}

pub mod table {
    use comfy_table::Cell;

    pub fn header(headers: &[&str]) -> Vec<Cell> {
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

    pub fn row(elements: &[&String]) -> Vec<Cell> {
        use comfy_table::Color;
        let mut cells = Vec::new();
        for cell in elements {
            cells.push(
                Cell::new(cell)
                    .fg(Color::White)
            );
        }
        cells
    }
}
