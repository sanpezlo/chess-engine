use std::{fmt, str::FromStr};
use thiserror::Error;

/// An error that can occur when parsing a [`Color`].
#[derive(Error, Debug)]
pub enum ColorError {
    /// The color is not valid.
    #[error("invalid color (expected w or b, got {0})")]
    Invalid(String),
}

/// A `Color` in chessboard.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    /// The color white.
    White,
    /// The color black.
    Black,
}

impl Color {
    /// Creates a new `Color` from a `u8`.
    ///
    /// # Panics
    ///
    /// Panics if the color is not 0 or 1.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::Color;
    /// let color = Color::new(0);
    /// assert_eq!(color, Color::White);
    /// ```
    pub fn new(color: u8) -> Self {
        match color {
            0 => Color::White,
            1 => Color::Black,
            _ => unreachable!(),
        }
    }
}

/// Parses a `Color` from a string.
///
/// # Errors
///
/// Returns a [`ColorError`] if the string is not a valid color.
///
/// # Examples
///
/// ```
/// # use chess_engine::Color;
/// let color: Color = "w".parse().unwrap();
/// assert_eq!(color, Color::White);
/// ```
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

/// Formats a `Color` as a string.
///
/// # Examples
///
/// ```
/// # use chess_engine::Color;
/// let color = Color::White;
/// assert_eq!(color.to_string(), "w");
/// ```
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Color::White => "w",
            Color::Black => "b",
        };

        write!(f, "{}", s)
    }
}
