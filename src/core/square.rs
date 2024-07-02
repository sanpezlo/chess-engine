use std::{fmt, str::FromStr};

use crate::{Color, File, FileError, Rank, RankError};
use thiserror::Error;

/// An error that can occur when parsing a [`Square`].
#[derive(Error, Debug)]
pub enum SquareError {
    /// The string is not 2 characters long.
    #[error("invalid length (expected 2, got {0})")]
    Length(usize),

    /// The rank is not valid.
    #[error("{0}")]
    Rank(#[from] RankError),

    /// The file is not valid.
    #[error("{0}")]
    File(#[from] FileError),
}

macro_rules! create_square {
    ($($square:ident),*) => {
        crate::core::macros::create_enum! {
            #[doc = concat!(
                "A `Square` on a chessboard.\n",
                "\n",
                "A `Square` is represented by a number from 0 to 63. The `Square` is\n",
                "calculated by multiplying the [`Rank`] by 8 and adding the [`File`]."
            )]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
            pub enum Square {
                $(
                    #[doc = concat!("The ", stringify!($square), " square.")]
                    $square
                ),*
            }
        }
    }
}

create_square! {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

impl Square {
    /// Creates a new `Square` from a [`File`] and a [`Rank`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Square, File, Rank};
    /// let square = Square::with_file_rank(File::F, Rank::Four);
    /// assert_eq!(square, Square::F4);
    /// ```
    pub fn with_file_rank(file: File, rank: Rank) -> Self {
        Self::new(rank as usize * 8 + file as usize)
    }

    /// Returns the [`File`] of the `Square`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Square, File, Rank};
    /// assert_eq!(Square::G5.file(), File::G);
    /// ```
    pub fn file(self) -> File {
        File::new(self as usize % 8)
    }

    /// Returns the [`Rank`] of the `Square`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Square, File, Rank};
    /// assert_eq!(Square::G5.rank(), Rank::Five);
    /// ```
    pub fn rank(self) -> Rank {
        Rank::new(self as usize / 8)
    }

    /// Returns the [`Color`] of the `Square`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Square, File, Rank, Color};
    /// assert_eq!(Square::A1.color(), Color::Black);
    /// ```
    pub fn color(self) -> Color {
        if self.rank() as u8 % 2 == 0 {
            if self.file() as u8 % 2 == 0 {
                Color::Black
            } else {
                Color::White
            }
        } else {
            if self.file() as u8 % 2 == 0 {
                Color::White
            } else {
                Color::Black
            }
        }
    }
}

/// The default `Square` is `Square::A1`.
///
/// # Examples
///
/// ```
/// # use chess_engine::Square;
/// assert_eq!(Square::default(), Square::A1);
/// ```
impl Default for Square {
    fn default() -> Self {
        Square::A1
    }
}

/// Parses a `Square` from a string.
///
/// # Errors
///
/// Returns a [`SquareError`] if the string is not a valid square.
///
/// # Examples
///
/// ```
/// # use chess_engine::{Square, File, Rank};
/// assert_eq!(
///    "h7".parse::<Square>().unwrap(),
///    Square::H7
/// );
/// ```
impl FromStr for Square {
    type Err = SquareError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.len() != 2 {
            return Err(SquareError::Length(str.len()));
        }

        let file = &str[0..1];
        let rank = &str[1..2];

        Ok(Square::with_file_rank(
            file.to_string().parse()?,
            rank.to_string().parse()?,
        ))
    }
}

/// Formats a `Square` as a string.
///
/// # Examples
///
/// ```
/// # use chess_engine::{Square, File, Rank};
/// assert_eq!(
///    Square::F2.to_string(),
///   "f2"
/// );
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}
