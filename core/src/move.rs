use crate::{PieceType, Square};

/// A move in a chess game.
pub struct Move {
    /// The square to move the piece from.
    pub from: Square,
    /// The square to move the piece to.
    pub to: Square,
    /// The piece to promote to, if any.
    pub promotion: Option<PieceType>,
}
