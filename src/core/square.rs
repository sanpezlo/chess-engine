use crate::{Color, File, FileError, Rank, RankError};
use std::{fmt, str::FromStr};
use thiserror::Error;

/// The number of squares on a chessboard.
pub const SQUARES: usize = 64;

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

    /// An unknown error occurred.
    #[error("uknown error")]
    Unknown,
}

/// A `Square` on a chessboard.
///
/// A `Square` is represented by a number from 0 to 63. The `Square` is
/// calculated by multiplying the [`Rank`] by 8 and adding the [`File`].
///
/// # Examples
///
/// ```
/// # use chess_engine::{Square, File, Rank, Color};
/// let square = Square::new(File::A, Rank::One);
/// assert_eq!(square.file(), File::A);
/// assert_eq!(square.rank(), Rank::One);
/// assert_eq!(square.color(), Color::Black);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Square(pub u8);

impl Square {
    /// Creates a new `Square` from a [`File`] and a [`Rank`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Square, File, Rank};
    /// let square = Square::new(File::A, Rank::One);
    /// assert_eq!(square, Square(0));
    /// ```
    pub fn new(file: File, rank: Rank) -> Self {
        Self(rank as u8 * 8 + file as u8)
    }

    /// Returns the [`File`] of the `Square`.
    ///
    /// # Panics
    ///
    /// Panics if the `Square` is not a legal square.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Square, File, Rank};
    /// assert_eq!(Square::new(File::A, Rank::One).file(), File::A);
    /// ```
    pub fn file(self) -> File {
        assert!(self.is_valid());

        File::new(self.0 % 8)
    }

    /// Returns the [`Rank`] of the `Square`.
    ///
    /// # Panics
    ///
    /// Panics if the `Square` is not a legal square.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Square, File, Rank};
    /// assert_eq!(Square::new(File::A, Rank::One).rank(), Rank::One);
    /// ```
    pub fn rank(self) -> Rank {
        assert!(self.is_valid());

        Rank::new(self.0 / 8)
    }

    /// Returns the [`Color`] of the `Square`.
    ///
    /// # Panics
    ///
    /// Panics if the `Square` is not a legal square.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Square, File, Rank, Color};
    /// assert_eq!(Square::new(File::A, Rank::One).color(), Color::Black);
    /// ```
    pub fn color(self) -> Color {
        assert!(self.is_valid());

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

    /// Returns `true` if the `Square` is a legal square.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Square, File, Rank};
    /// assert!(!Square(64).is_valid());
    /// ```
    pub fn is_valid(self) -> bool {
        self.0 < SQUARES as u8
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
///    "a1".parse::<Square>().unwrap(),
///    Square::new(File::A, Rank::One)
/// );
/// ```
impl FromStr for Square {
    type Err = SquareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        if s.len() != 2 {
            return Err(SquareError::Length(s.len()));
        }

        let file = chars.next().ok_or(SquareError::Unknown)?;
        let rank = chars.next().ok_or(SquareError::Unknown)?;

        Ok(Square::new(
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
///    Square::new(File::A, Rank::One).to_string(),
///   "a1"
/// );
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}
