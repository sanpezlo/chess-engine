use std::{fmt, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("the color `{0}` is not valid")]
    Invalid(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn new(color: u8) -> Self {
        match color {
            0 => Color::White,
            1 => Color::Black,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Color {
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err(ColorError::Invalid(s.to_string())),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Color::White => "w",
            Color::Black => "b",
        };

        write!(f, "{}", s)
    }
}
