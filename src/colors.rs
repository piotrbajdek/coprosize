use std::fmt::Display;

pub const RESET: &str = "\x1b[0m";
pub const RED: &str = "\x1b[31m";
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
pub const CYAN: &str = "\x1b[36m";
pub const BLUE_UNDERLINED: &str = "\x1b[34;4m";

pub struct ColoredText<'a> {
    color: &'a str,
    text: &'a str,
}

impl<'a> Display for ColoredText<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color)?;
        f.pad(self.text)?;
        write!(f, "{RESET}")
    }
}

pub fn colorize<'a>(color: &'a str, text: &'a str) -> ColoredText<'a> {
    ColoredText { color, text }
}
