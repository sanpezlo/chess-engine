use super::macros::{create_enum, enum_str};

create_enum! {
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
