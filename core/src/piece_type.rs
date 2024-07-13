use core::fmt;

use super::macros::{create_enum, enum_str};

create_enum! {
    /// A `PieceType` in chess.
    #[derive(Clone, Copy, PartialEq, Eq)]
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
        King
    }
}

enum_str! {
    PieceType, PieceTypeError {
        Pawn = "p",
        Knight = "n",
        Bishop = "b",
        Rook = "r",
        Queen = "q",
        King = "k"
    }
}

impl fmt::Debug for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece = match self {
            PieceType::King => '♔',
            PieceType::Queen => '♕',
            PieceType::Rook => '♖',
            PieceType::Bishop => '♗',
            PieceType::Knight => '♘',
            PieceType::Pawn => '♙',
        };

        write!(f, "{}", piece)
    }
}
