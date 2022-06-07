//! Terminal utilities
use termion::color::{Fg, LightCyan, Reset};
use termion::style::Bold;
pub const RESET: Fg<Reset> = Fg(Reset);
pub const PRIMARY: Fg<LightCyan> = Fg(LightCyan);
pub const BOLD: Bold = Bold;
pub const DEC_RESET: termion::style::Reset = termion::style::Reset;
