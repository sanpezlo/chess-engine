use core::fmt;

use crate::{BitBoard, File, Piece, Rank, Square};

/// Represents multiple moves for a piece.
pub struct PieceMoves {
    /// The [`Piece`] for which moves are generated.
    pub piece: Piece,
    /// The square to move the piece from.
    pub from: Square,
    /// The multiple destination squares to move the piece to.
    pub to: BitBoard,
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
