use super::macros::{create_enum, enum_str};

create_enum! {
    /// A `SlidingPiece` in chess.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SlidingPiece  {
        /// A Bishop.
        Bishop,
        /// A Rook.
        Rook,
        /// A Queen.
        Queen
    }
}

enum_str! {
    SlidingPiece, SlidingPieceError {
        Bishop = "b",
        Rook = "r",
        Queen = "q"
    }
}
