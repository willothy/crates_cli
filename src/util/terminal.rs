//! Terminal utilities
use std::fmt::Display;
use crossterm::style::{Stylize, Attribute, ContentStyle, StyledContent};

pub trait CratesCliStyle<T: Stylize + Display> {
    fn style_primary(self) -> StyledContent<T>;
    fn style_secondary(self) -> StyledContent<T>;
}

impl<T: Stylize + Display> CratesCliStyle<T> for T {
    fn style_primary(self) -> StyledContent<T> {
        ContentStyle::new().attribute(Attribute::Bold).cyan().apply(self)
    }

    fn style_secondary(self) -> StyledContent<T> {
        ContentStyle::new().cyan().apply(self)
    }
}