use termion::{terminal_size};
use std::fmt::{Display, Formatter};
use std::fmt;

pub enum ColorError {
    InvalidColor(String),
}

impl Display for ColorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ColorError::InvalidColor(c) => write!(f, "invalid color {}", c)
        }
    }
}

pub enum Color {
    Red,
    Green,
    Orange,
    Blue,
    Purple,
    BlueGreen,
    White,
    Gray,
    DarkRed,
    BrightGreen,
    Pink,
}

impl Color {
    pub fn to_number(&self) -> u8 {
        match self {
            Color::Red => 1,
            Color::Green => 2,
            Color::Orange => 3,
            Color::Blue => 4,
            Color::Purple => 5,
            Color::BlueGreen => 6,
            Color::White => 7,
            Color::Gray => 8,
            Color::DarkRed => 9,
            Color::BrightGreen => 34,
            Color::Pink => 200,
        }
    }

    pub fn from_string(str: &str) -> Result<Color, ColorError> {
        Ok(match str {
            "red" => Color::Red,
            "green" => Color::Green,
            "orange" => Color::Orange,
            "blue" => Color::Blue,
            "purple" => Color::Purple,
            "bluegreen" => Color::BlueGreen,
            "white" => Color::White,
            "gray" => Color::Gray,
            "darkred" => Color::DarkRed,
            "brightgreen" => Color::BrightGreen,
            "pink" => Color::Pink,
            _ => return Err(ColorError::InvalidColor(str.to_string()))
        })
    }
}

pub fn test_colors() {
    let char_size: u16 = 4;
    let mut chars_per_line: u8 = 10;

    if let Ok((w, _)) = terminal_size() {
        chars_per_line = ((w - (w % char_size)) / char_size) as u8;
    }

    // Foreground colors
    println!("Foreground:");
    for i in 1..=255 {
        print!("{}{}", fg_color_code(i), format!("{:<width$}", i, width=char_size as usize));
        if i % chars_per_line == 0 && i > 0 {
            println!();
        }
    }
    print!("{}", reset_color());
    println!();
    println!("Background:");
    // Background colors
    for i in 1..=255 {
        print!("{}{}", bg_color_code(i), format!("{:<width$}", i, width=char_size as usize));
        if i % chars_per_line == 0 && i > 0 {
            println!();
        }
    }
    print!("{}", reset_color());
    println!();
}

/// Returns the formatting string required to set the coming
/// foreground color to the one specified by the color_code.
pub fn fg_color_code(color_code: u8) -> String {
    format!("\x1b[38;5;{}m", color_code)
}

/// Returns the formatting string required to set the coming
/// foreground color to the one specified by the color.
pub fn fg_color(color: Color) -> String {
    fg_color_code(color.to_number())
}

/// Returns the formatting string required to set the coming
/// background color to the one specified by the color_code.
pub fn bg_color_code(color_code: u8) -> String {
    format!("\x1b[48;5;{}m", color_code)
}

/// Returns the formatting string required to set the coming
/// background color to the one specified by the color.
pub fn bg_color(color: Color) -> String {
    bg_color_code(color.to_number())
}

pub fn reset_color() -> String {
    String::from("\x1b[0m")
}