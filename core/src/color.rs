use super::macros::{create_enum, enum_str};

create_enum! {
    /// A `Color` in chessboard.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Color {
        /// The color white.
        White,
        /// The color black.
        Black
    }
}

enum_str!(
    Color, ColorError {
        White = "w",
        Black = "b"
    }
);

/// The default `Color` is `Color::White`.
///
/// # Examples
///
/// ```
/// # use chess_engine_core::*;
/// assert_eq!(Color::default(), Color::White);
/// ```
impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

impl std::ops::Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
