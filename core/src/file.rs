use crate::{bitboard, BitBoard};

use super::macros::{create_enum, enum_str};

create_enum! {
    /// A `File` on a chessboard.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum File {
        /// The File A.
        A,
        /// The File B.
        B,
        /// The File C.
        C,
        /// The File D.
        D,
        /// The File E.
        E,
        /// The File F.
        F,
        /// The File G.
        G,
        /// The File H.
        H
    }
}

enum_str! {
    File, FileError {
        A = "a",
        B = "b",
        C = "c",
        D = "d",
        E = "e",
        F = "f",
        G = "g",
        H = "h"
    }
}

impl File {
    /// Converts a [`File`] to a [`BitBoard`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = File::C.bitboard();
    /// assert_eq!(bitboard, bitboard! {
    ///     . . X . . . . .
    ///     . . X . . . . .
    ///     . . X . . . . .
    ///     . . X . . . . .
    ///     . . X . . . . .
    ///     . . X . . . . .
    ///     . . X . . . . .
    ///     . . X . . . . .
    /// });
    /// ```
    pub const fn bitboard(self) -> BitBoard {
        const BITBOARD: u64 = bitboard! {
            X . . . . . . .
            X . . . . . . .
            X . . . . . . .
            X . . . . . . .
            X . . . . . . .
            X . . . . . . .
            X . . . . . . .
            X . . . . . . .
        }
        .0;

        BitBoard(BITBOARD << self as usize)
    }
}
