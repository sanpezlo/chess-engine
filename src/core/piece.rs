use crate::{Color, Player};
use std::{fmt, str::FromStr};
use thiserror::Error;

/// The maximum number of pieces per player in chess.
pub const MAX_PIECES_PER_PLAYER: usize = 16;

/// The maximum number of pawns per player in chess.
pub const MAX_PAWNS_PER_PLAYER: usize = 8;

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
/// A `Piece` is represented by a [`PieceType`] and a [`Player`].
///
/// # Examples
///
/// ```
/// # use chess_engine::{Piece, PieceType, Player, Color};
/// let piece = Piece::new(PieceType::Pawn, Player(Color::White));
/// assert_eq!(piece.piece_type(), PieceType::Pawn);
/// assert_eq!(piece.player(), Player(Color::White));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    piece_type: PieceType,
    player: Player,
}

impl Piece {
    /// Creates a new `Piece` from a [`PieceType`] and a [`Player`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Piece, PieceType, Player, Color};
    /// let piece = Piece::new(PieceType::Pawn, Player(Color::White));
    /// assert_eq!(piece.piece_type(), PieceType::Pawn);
    /// assert_eq!(piece.player(), Player(Color::White));
    /// ```
    pub fn new(piece_type: PieceType, player: Player) -> Self {
        Piece { piece_type, player }
    }

    /// Returns the [`PieceType`] of the `Piece`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Piece, PieceType, Player, Color};
    /// let piece = Piece::new(PieceType::Pawn, Player(Color::White));
    /// assert_eq!(piece.piece_type(), PieceType::Pawn);
    /// ```
    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    /// Returns the [`Player`] of the `Piece`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Piece, PieceType, Player, Color};
    /// let piece = Piece::new(PieceType::Pawn, Player(Color::White));
    /// assert_eq!(piece.player(), Player(Color::White));
    /// ```
    pub fn player(&self) -> Player {
        self.player
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
/// # use chess_engine::{Piece, PieceType, Player, Color};
/// let piece: Piece = "P".parse().unwrap();
/// assert_eq!(piece, Piece::new(PieceType::Pawn, Player(Color::White)));
/// ```
impl FromStr for Piece {
    type Err = PieceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let (piece_type, player) = match chars.next() {
            Some('P') => (PieceType::Pawn, Player(Color::White)),
            Some('N') => (PieceType::Knight, Player(Color::White)),
            Some('B') => (PieceType::Bishop, Player(Color::White)),
            Some('R') => (PieceType::Rook, Player(Color::White)),
            Some('Q') => (PieceType::Queen, Player(Color::White)),
            Some('K') => (PieceType::King, Player(Color::White)),
            Some('p') => (PieceType::Pawn, Player(Color::Black)),
            Some('n') => (PieceType::Knight, Player(Color::Black)),
            Some('b') => (PieceType::Bishop, Player(Color::Black)),
            Some('r') => (PieceType::Rook, Player(Color::Black)),
            Some('q') => (PieceType::Queen, Player(Color::Black)),
            Some('k') => (PieceType::King, Player(Color::Black)),
            _ => return Err(PieceError::Invalid(s.to_string())),
        };

        Ok(Piece::new(piece_type, player))
    }
}

/// Formats a `Piece` as a string.
///
/// # Examples
///
/// ```
/// # use chess_engine::{Piece, PieceType, Player, Color};
/// let piece = Piece::new(PieceType::Pawn, Player(Color::Black));
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

        if self.player == Player(Color::White) {
            write!(f, "{}", piece_type.to_uppercase())
        } else {
            write!(f, "{}", piece_type)
        }
    }
}
