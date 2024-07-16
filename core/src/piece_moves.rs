use core::fmt;

use crate::{BitBoard, File, Move, Piece, PieceType, Rank, Square};

/// Represents multiple moves for a piece.
#[derive(Clone, Copy)]
pub struct PieceMoves {
    /// The [`Piece`] for which moves are generated.
    piece: Piece,
    /// The square to move the piece from.
    from: Square,
    /// The multiple destination squares to move the piece to.
    to: BitBoard,
}

impl PieceMoves {
    /// Creates a new `PieceMoves`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let piece = Piece::new(PieceType::Pawn, Color::White);
    /// let from = Square::A2;
    /// let to = bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     X . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// };
    /// let moves = PieceMoves::new(piece, from, to);
    /// ```
    pub fn new(piece: Piece, from: Square, to: BitBoard) -> Self {
        Self { piece, from, to }
    }

    /// Returns the available moves for a piece.
    pub fn moves(&self) -> Vec<Move> {
        self.into_iter().collect()
    }
}

impl IntoIterator for PieceMoves {
    type Item = Move;
    type IntoIter = PieceMovesIter;

    fn into_iter(self) -> Self::IntoIter {
        PieceMovesIter {
            moves: self,
            promotion: 0,
        }
    }
}

/// Iterator over the moves for a piece.
pub struct PieceMovesIter {
    moves: PieceMoves,
    promotion: u8,
}

impl Iterator for PieceMovesIter {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let to: Square = self.moves.to.least_significant_square()?;
        self.moves.to = self.moves.to.unset_square(to);

        let promotion = if self.moves.piece.piece_type() == PieceType::Pawn
            && self.promotion < 3
            && (to.rank() == Rank::One || to.rank() == Rank::Eight)
        {
            let promotion = match self.promotion {
                0 => Some(PieceType::Knight),
                1 => Some(PieceType::Bishop),
                2 => Some(PieceType::Rook),
                3 => Some(PieceType::Queen),
                _ => unreachable!(),
            };

            self.promotion += 1;

            promotion
        } else {
            self.promotion = 0;
            None
        };

        Some(Move::new(self.moves.from, to, promotion))
    }
}

impl fmt::Display for PieceMoves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("\n");

        s.push_str(&format!("{} ", self.from));

        s.push_str(&format!("{} ", self.piece));

        for to_square in self.to {
            s.push_str(&format!("{} ", to_square));
        }

        write!(f, "{}", s)
    }
}

impl fmt::Debug for PieceMoves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("\n");

        for rank in Rank::ALL.into_iter().rev() {
            s.push_str(&format!("  {} ", rank));

            'file: for file in File::ALL {
                let square = Square::with_file_rank(file, rank);

                if square == self.from {
                    s.push_str(&format!("{:?} ", self.piece));
                } else {
                    for to_square in self.to {
                        if to_square == square {
                            s.push_str("X ");
                            continue 'file;
                        }
                    }

                    s.push_str(". ");
                }
            }

            s.push_str("\n");
        }

        s.push_str("\n    a b c d e f g h");

        write!(f, "{}", s)
    }
}
