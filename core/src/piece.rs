use crate::{Color, PieceType, PieceTypeError};
use std::{fmt, str::FromStr};

/// A `Piece` in chess.
///
/// A `Piece` is represented by a [`PieceType`] and a [`Color`].
///
/// # Examples
///
/// ```
/// # use chess_engine_core::*;
/// let piece = Piece::new(PieceType::Pawn, Color::White);
/// assert_eq!(piece.piece_type(), PieceType::Pawn);
/// assert_eq!(piece.color(), Color::White);
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    /// The maximum number of pieces per color in chess.
    pub const MAX_PIECES_PER_COLOR: usize = 16;

    /// The maximum number of pawns per color in chess.
    pub const MAX_PAWNS_PER_COLOR: usize = 8;

    /// The number of pieces in chess.
    pub const LEN: usize = PieceType::LEN * Color::LEN;

    /// An array of all the pieces in chess.
    pub const ALL: [Self; Self::LEN] = {
        let mut pieces = [Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }; Self::LEN];

        let mut color_index = 0;

        while color_index < Color::LEN {
            let mut piece_type_index = 0;
            while piece_type_index < PieceType::LEN {
                pieces[piece_type_index + color_index * PieceType::LEN] = Piece {
                    piece_type: PieceType::new(piece_type_index),
                    color: Color::new(color_index),
                };
                piece_type_index += 1;
            }
            color_index += 1;
        }

        pieces
    };

    /// Creates a new `Piece` from a [`PieceType`] and a [`Color`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
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
    /// # use chess_engine_core::*;
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
    /// # use chess_engine_core::*;
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
/// # use chess_engine_core::*;
/// let piece: Piece = "P".parse().unwrap();
/// assert_eq!(piece, Piece::new(PieceType::Pawn, Color::White));
/// ```
impl FromStr for Piece {
    type Err = PieceTypeError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let lowercase = str.to_lowercase();
        let piece_type = PieceType::from_str(&lowercase)?;

        if str == lowercase {
            Ok(Piece::new(piece_type, Color::Black))
        } else {
            Ok(Piece::new(piece_type, Color::White))
        }
    }
}

/// Formats a `Piece` as a string.
///
/// # Examples
///
/// ```
/// # use chess_engine_core::*;
/// let piece = Piece::new(PieceType::Pawn, Color::Black);
/// assert_eq!(piece.to_string(), "p");
/// ```
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece_type = self.piece_type.to_string();

        if self.color == Color::White {
            write!(f, "{}", piece_type.to_uppercase())
        } else {
            write!(f, "{}", piece_type)
        }
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bytes: Vec<u8> = format!("{:?}", self.piece_type).bytes().collect();

        if self.color == Color::White {
            write!(f, "{}", String::from_utf8(bytes).unwrap())
        } else {
            bytes[2] += 6;
            write!(f, "{}", String::from_utf8(bytes).unwrap())
        }
    }
}
