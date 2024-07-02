use crate::Color;
use std::{fmt, str::FromStr};
use thiserror::Error;

/// The maximum number of pieces per color in chess.
pub const MAX_PIECES_PER_COLOR: usize = 16;

/// The maximum number of pawns per color in chess.
pub const MAX_PAWNS_PER_COLOR: usize = 8;

/// The number of piece types in chess.
pub const PIECE_TYPES: usize = 6;

/// A `PieceType` in chess.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    /// A Pawn.
    Pawn,
    /// A Knight.
    Knight,
    /// A Bishop.
    Bishop,
    /// A Rook.
    Rook,
    /// A Queen.
    Queen,
    /// A King.
    King,
}

impl PieceType {
    /// Creates a new `PieceType` from a `u8`.
    ///
    /// # Panics
    ///
    /// Panics if the piece type is not 0-5.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::PieceType;
    /// let piece_type = PieceType::new(0);
    /// assert_eq!(piece_type, PieceType::Pawn);
    /// ```
    pub fn new(piece: u8) -> Self {
        match piece {
            0 => PieceType::Pawn,
            1 => PieceType::Knight,
            2 => PieceType::Bishop,
            3 => PieceType::Rook,
            4 => PieceType::Queen,
            5 => PieceType::King,
            _ => unreachable!(),
        }
    }
}

/// An error that can occur when parsing a [`PieceType`].
#[derive(Error, Debug)]
pub enum PieceError {
    /// The piece is not valid.
    #[error("invalid piece (expected P, N, B, R, Q, or K, got {0})")]
    Invalid(String),
}

/// A `Piece` in chess.
///
/// A `Piece` is represented by a [`PieceType`] and a [`Color`].
///
/// # Examples
///
/// ```
/// # use chess_engine::{Piece, PieceType, Color};
/// let piece = Piece::new(PieceType::Pawn, Color::White);
/// assert_eq!(piece.piece_type(), PieceType::Pawn);
/// assert_eq!(piece.color(), Color::White);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    /// Creates a new `Piece` from a [`PieceType`] and a [`Color`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Piece, PieceType, Color};
    /// let piece = Piece::new(PieceType::Pawn, Color::White);
    /// assert_eq!(piece.piece_type(), PieceType::Pawn);
    /// assert_eq!(piece.color(), Color::White);
    /// ```
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }

    /// Returns the [`PieceType`] of the `Piece`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Piece, PieceType, Color};
    /// let piece = Piece::new(PieceType::Pawn, Color::White);
    /// assert_eq!(piece.piece_type(), PieceType::Pawn);
    /// ```
    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    /// Returns the [`Color`] of the `Piece`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Piece, PieceType, Color};
    /// let piece = Piece::new(PieceType::Pawn, Color::White);
    /// assert_eq!(piece.color(), Color::White);
    /// ```
    pub fn color(&self) -> Color {
        self.color
    }
}

/// Parses a `Piece` from a string.
///
/// # Errors
///
/// Returns a [`PieceError`] if the string is not a valid piece.
///
/// # Examples
///
/// ```
/// # use chess_engine::{Piece, PieceType, Color};
/// let piece: Piece = "P".parse().unwrap();
/// assert_eq!(piece, Piece::new(PieceType::Pawn, Color::White));
/// ```
impl FromStr for Piece {
    type Err = PieceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let (piece_type, color) = match chars.next() {
            Some('P') => (PieceType::Pawn, Color::White),
            Some('N') => (PieceType::Knight, Color::White),
            Some('B') => (PieceType::Bishop, Color::White),
            Some('R') => (PieceType::Rook, Color::White),
            Some('Q') => (PieceType::Queen, Color::White),
            Some('K') => (PieceType::King, Color::White),
            Some('p') => (PieceType::Pawn, Color::Black),
            Some('n') => (PieceType::Knight, Color::Black),
            Some('b') => (PieceType::Bishop, Color::Black),
            Some('r') => (PieceType::Rook, Color::Black),
            Some('q') => (PieceType::Queen, Color::Black),
            Some('k') => (PieceType::King, Color::Black),
            _ => return Err(PieceError::Invalid(s.to_string())),
        };

        Ok(Piece::new(piece_type, color))
    }
}

/// Formats a `Piece` as a string.
///
/// # Examples
///
/// ```
/// # use chess_engine::{Piece, PieceType, Color};
/// let piece = Piece::new(PieceType::Pawn, Color::Black);
/// assert_eq!(piece.to_string(), "p");
/// ```
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece_type = match self.piece_type {
            PieceType::Pawn => "p",
            PieceType::Knight => "n",
            PieceType::Bishop => "b",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "k",
        };

        if self.color == Color::White {
            write!(f, "{}", piece_type.to_uppercase())
        } else {
            write!(f, "{}", piece_type)
        }
    }
}
