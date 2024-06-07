use crate::{File, Rank, Square};
use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

/// A representation of a chessboard as a 64-bit unsigned integer.
///
/// The bits are indexed from a1 to h8, with a1 being the least significant bit
/// (rightmost bit) and h8 being the most significant bit (leftmost bit).
///
/// # Examples
///
/// ```
/// # use chess_engine::{BitBoard};
/// let bitboard = BitBoard(0x000000000000FF00);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BitBoard(pub u64);

/// Performs a bitwise AND operation on two `BitBoard`s.
///
/// # Examples
///
/// ```
/// # use chess_engine::BitBoard;
/// let bitboard1 = BitBoard(0x000000000000FF00);
/// let bitboard2 = BitBoard(0x0000000000000FFF);
/// let result = bitboard1 & bitboard2;
/// assert_eq!(result, BitBoard(0x0000000000000F00));
/// ```
impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

/// Performs a bitwise AND assignment operation on two `BitBoard`s.
///
/// # Examples
///
/// ```
/// # use chess_engine::BitBoard;
/// let mut bitboard1 = BitBoard(0x000000000000FF00);
/// bitboard1 &= BitBoard(0x0000000000000FFF);
/// assert_eq!(bitboard1, BitBoard(0x0000000000000F00));
impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

/// Performs a bitwise OR operation on two `BitBoard`s.
///
/// # Examples
///
/// ```
/// # use chess_engine::BitBoard;
/// let bitboard1 = BitBoard(0x000000000000FF00);
/// let bitboard2 = BitBoard(0x0000000000000FFF);
/// let result = bitboard1 | bitboard2;
/// assert_eq!(result, BitBoard(0x000000000000FFFF));
impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

/// Performs a bitwise OR assignment operation on two `BitBoard`s.
///
/// # Examples
///
/// ```
/// # use chess_engine::BitBoard;
/// let mut bitboard1 = BitBoard(0x000000000000FF00);
/// bitboard1 |= BitBoard(0x0000000000000FFF);
/// assert_eq!(bitboard1, BitBoard(0x000000000000FFFF));
/// ```
impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Performs a bitwise XOR operation on two `BitBoard`s.
///
/// # Examples
///
/// ```
/// # use chess_engine::BitBoard;
/// let bitboard1 = BitBoard(0x000000000000FF00);
/// let bitboard2 = BitBoard(0x0000000000000FFF);
/// let result = bitboard1 ^ bitboard2;
/// assert_eq!(result, BitBoard(0x000000000000F0FF));
impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

/// Performs a bitwise XOR assignment operation on two `BitBoard`s.
///
/// # Examples
///
/// ```
/// # use chess_engine::BitBoard;
/// let mut bitboard1 = BitBoard(0x000000000000FF00);
/// bitboard1 ^= BitBoard(0x0000000000000FFF);
/// assert_eq!(bitboard1, BitBoard(0x000000000000F0FF));
/// ```
impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

/// Performs a bitwise NOT operation on a `BitBoard`.
///
/// # Examples
///
/// ```
/// # use chess_engine::BitBoard;
/// let bitboard = BitBoard(0x000000000000FF00);
/// let result = !bitboard;
/// assert_eq!(result, BitBoard(0xFFFFFFFFFFFF00FF));
impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

/// Converts a [`Square`] to a [`BitBoard`].
///
/// # Panics
///
/// Panics if the [`Square`] is not a legal square.
///
/// # Examples
///
/// ```
/// # use chess_engine::{BitBoard, Square, File, Rank};
/// let bitboard: BitBoard = Square::new(File::A, Rank::One).into();
/// ```
impl From<Square> for BitBoard {
    fn from(square: Square) -> Self {
        assert!(square.is_valid());

        Self(1u64 << square.0)
    }
}

/// Formats a `BitBoard` as a string.
///
/// The string is formatted as a 8x8 grid of `X` and `.` characters, with `X`
/// representing a set bit and `.` representing an unset bit.
///
/// # Examples
///
/// ```
/// # use chess_engine::BitBoard;
/// let bitboard = BitBoard(0x000000000000FF00);
/// println!("{}", bitboard);
/// ```
///
/// ```textplain
///   8  . . . . . . . .
///   7  . . . . . . . .
///   6  . . . . . . . .
///   5  . . . . . . . .
///   4  . . . . . . . .
///   3  . . . . . . . .
///   2  X X X X X X X X
///   1  . . . . . . . .
///      a b c d e f g h
///       
///    000000000000FF00
/// ```
impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("\n");

        for rank in (0..8u8).rev() {
            let rank = Rank::new(rank);
            s.push_str(&format!("  {} ", rank));

            for file in 0..8u8 {
                let file = File::new(file);

                if BitBoard::from(Square::new(file, rank)).0 & self.0 != 0 {
                    s.push_str("X ");
                } else {
                    s.push_str(". ");
                }
            }

            s.push_str("\n");
        }

        s.push_str("\n     a b c d e f g h\n\n");

        s.push_str(&format!("   {:016X}\n", self.0));

        write!(f, "{}", s)
    }
}
