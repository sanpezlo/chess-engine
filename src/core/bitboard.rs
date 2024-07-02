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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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

        for rank in (0..Rank::LEN).rev() {
            let rank = Rank::new(rank);
            s.push_str(&format!("  {} ", rank));

            for file in 0..File::LEN {
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

/// An iterator over the set bits of a `BitBoard`.
///
/// # Examples
///
/// ```
/// # use chess_engine::{BitBoard, Square, File, Rank};
/// let bitboard = BitBoard(0x000000000000FF00);
/// let mut iter = bitboard.into_iter();
/// assert_eq!(iter.next(), Some(Square::new(File::A, Rank::Two)));
/// assert_eq!(iter.next(), Some(Square::new(File::B, Rank::Two)));
/// ```
pub struct BitBoardIter {
    bitboard: BitBoard,
}

/// Implements the `Iterator` trait for `BitBoardIter`.
///
/// The iterator yields the set bits of a `BitBoard`.
impl Iterator for BitBoardIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bitboard == BitBoard(0) {
            return None;
        }

        let trailing_zeros = self.bitboard.0.trailing_zeros();
        self.bitboard.0 ^= 1 << trailing_zeros;

        Some(Square(trailing_zeros as u8))
    }
}

/// Converts a `BitBoard` into an iterator over its set bits.
///
/// # Examples
///
/// ```no_run
/// # use chess_engine::{BitBoard, Square, File, Rank};
/// let bitboard = BitBoard(0x000000000000FF00);
/// let mut iter = bitboard.into_iter();
/// assert_eq!(iter.next(), Some(Square::new(File::A, Rank::Two)));
/// assert_eq!(iter.next(), Some(Square::new(File::B, Rank::Two)));
/// ```
impl IntoIterator for BitBoard {
    type Item = Square;
    type IntoIter = BitBoardIter;

    fn into_iter(self) -> Self::IntoIter {
        BitBoardIter { bitboard: self }
    }
}

impl BitBoard {
    /// An empty `BitBoard`.
    pub const EMPTY: Self = Self(0);
}

/// A macro for creating a bitboard.
///
/// # Examples
///
/// ```
/// # use chess_engine::bitboard;
/// let bb = bitboard! {
///     X X X X X X X X
///     X . . . . . . X
///     X . . . . . . X
///     X . . . . . . X
///     X . . . . . . X
///     X . . . . . . X
///     X . . . . . . X
///     X X X X X X X X
/// };
/// ```
#[macro_export]
macro_rules! bitboard {
    (@is_valid_bitboard
        $a8:tt $b8:tt $c8:tt $d8:tt $e8:tt $f8:tt $g8:tt $h8:tt
        $a7:tt $b7:tt $c7:tt $d7:tt $e7:tt $f7:tt $g7:tt $h7:tt
        $a6:tt $b6:tt $c6:tt $d6:tt $e6:tt $f6:tt $g6:tt $h6:tt
        $a5:tt $b5:tt $c5:tt $d5:tt $e5:tt $f5:tt $g5:tt $h5:tt
        $a4:tt $b4:tt $c4:tt $d4:tt $e4:tt $f4:tt $g4:tt $h4:tt
        $a3:tt $b3:tt $c3:tt $d3:tt $e3:tt $f3:tt $g3:tt $h3:tt
        $a2:tt $b2:tt $c2:tt $d2:tt $e2:tt $f2:tt $g2:tt $h2:tt
        $a1:tt $b1:tt $c1:tt $d1:tt $e1:tt $f1:tt $g1:tt $h1:tt
    ) => {};
    (@is_valid_bitboard $($token:tt)*) => {
        compile_error!("Expected 64 squares")
    };
    (@is_valid_square X) => {
        true
    };
    (@is_valid_square .) => {
        false
    };
    (@is_valid_square $token:tt) => {
        compile_error!(concat!(
            "Expected only `X` or `.` tokens, found `",
            stringify!($token),
            "`"
        ))
    };
    ($($token:tt)*) => {{
        $crate::bitboard! { @is_valid_bitboard $($token)* }
        const BITBOARD: $crate::BitBoard = {
            let mut bitboard = $crate::BitBoard::EMPTY;
            let mut index = 0;

            $(
                if $crate::bitboard!(@is_valid_square $token) {
                    bitboard.0 |= 1 << index;
                }
                index += 1;
            )*

            bitboard
        };
        BITBOARD
    }};
}
