use crate::{bitboard, BitBoard};

use super::macros::{create_enum, enum_str};

create_enum! {
    /// A `Rank` on a chessboard.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Rank {
        /// The Rank 1.
        One,
        /// The Rank 2.
        Two,
        /// The Rank 3.
        Three,
        /// The Rank 4.
        Four,
        /// The Rank 5.
        Five,
        /// The Rank 6.
        Six,
        /// The Rank 7.
        Seven,
        /// The Rank 8.
        Eight
    }
}

enum_str! {
    Rank, RankError {
        One = "1",
        Two =  "2",
        Three = "3",
        Four = "4",
        Five = "5",
        Six = "6",
        Seven = "7",
        Eight = "8"
    }
}

impl Rank {
    /// Converts a [`Rank`] to a [`BitBoard`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = Rank::Three.bitboard();
    /// assert_eq!(bitboard, bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     X X X X X X X X
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// });
    /// ```
    pub const fn bitboard(self) -> BitBoard {
        const BITBOARD: u64 = bitboard! {
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            X X X X X X X X
        }
        .0;

        BitBoard(BITBOARD << (self as usize * 8))
    }
}
