//! Terminal utilities
use crossterm::{
    style::{Attribute, ContentStyle, Print, StyledContent, Stylize},
    QueueableCommand, terminal::SetTitle,
};
use std::fmt::Display;
use std::io::{self, stdout, Write};

pub trait CratesCliStyle<T: Stylize + Display> {
    fn style_primary(self) -> StyledContent<T>;
    fn style_secondary(self) -> StyledContent<T>;
}

impl<T: Stylize + Display> CratesCliStyle<T> for T {
    fn style_primary(self) -> StyledContent<T> {
        ContentStyle::new()
            .attribute(Attribute::Bold)
            .cyan()
            .apply(self)
    }

    fn style_secondary(self) -> StyledContent<T> {
        ContentStyle::new().cyan().apply(self)
    }
}

pub fn print_queue<T: Into<String> + Display>(values: Vec<T>, new_line: bool) -> io::Result<()> {
    let mut stdout = stdout();
    for val in values {
        stdout.queue(Print(val.to_string()))?;
    }
    if new_line {
        stdout.queue(Print("\n"))?;
    }
    stdout.flush()
}
pub fn print<T: Into<String> + Display>(value: T) -> Result<(), std::io::Error> {
    stdout().queue(Print(value))?.flush()
}

pub fn print_error<T: Into<String> + Display>(value: T) -> Result<(), std::io::Error> {
    stdout().queue(Print(value.to_string().red()))?.flush()
}

pub fn set_title<T: Into<String> + Display>(title: T) {
    crossterm::execute!(std::io::stdout(), SetTitle(title.to_string())).unwrap_or(());
}
