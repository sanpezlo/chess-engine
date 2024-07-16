use std::fmt;

use crate::{PieceType, Square};

/// A move in a chess game.
#[derive(Clone, Copy)]
pub struct Move {
    /// The square to move the piece from.
    from: Square,
    /// The square to move the piece to.
    to: Square,
    /// The piece to promote to, if any.
    promotion: Option<PieceType>,
}

impl Move {
    /// Creates a new move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let from = Square::A2;
    /// let to = Square::A3;
    /// let promotion = None;
    /// let mv = Move::new(from, to, promotion);
    /// ```
    pub const fn new(from: Square, to: Square, promotion: Option<PieceType>) -> Self {
        Self {
            from,
            to,
            promotion,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.from,
            self.to,
            self.promotion.map_or("".to_string(), |s| s.to_string())
        )
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bitboard = self.from.bitboard().set_square(self.to);

        write!(
            f,
            "{}\n\n   {}",
            format!("{:?}", bitboard),
            self.to_string()
        )
    }
}
