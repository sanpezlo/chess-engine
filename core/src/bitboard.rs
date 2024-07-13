use crate::{File, Rank, Square};
use std::{
    fmt::{self, Debug},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

/// A macro for creating a bitboard.
///
/// # Examples
///
/// ```
/// # use chess_engine_core::*;
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
    ) => {
        $crate::bitboard! {
            @bitboard
            $a1 $b1 $c1 $d1 $e1 $f1 $g1 $h1
            $a2 $b2 $c2 $d2 $e2 $f2 $g2 $h2
            $a3 $b3 $c3 $d3 $e3 $f3 $g3 $h3
            $a4 $b4 $c4 $d4 $e4 $f4 $g4 $h4
            $a5 $b5 $c5 $d5 $e5 $f5 $g5 $h5
            $a6 $b6 $c6 $d6 $e6 $f6 $g6 $h6
            $a7 $b7 $c7 $d7 $e7 $f7 $g7 $h7
            $a8 $b8 $c8 $d8 $e8 $f8 $g8 $h8
        }
    };
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
    (@bitboard $($token:tt)*) => {{
        const BITBOARD: $crate::BitBoard = {
            let mut index = 0;
            let mut bitboard = $crate::BitBoard::EMPTY;

            $(
                index += 1;
                if $crate::bitboard!(@is_valid_square $token) {
                    bitboard = bitboard.set_square($crate::Square::new(index - 1));
                }
            )*


            bitboard
        };
        BITBOARD
    }};
    ($($token:tt)*) => {{
        $crate::bitboard! { @is_valid_bitboard $($token)* }
    }};
}

/// A representation of a chessboard as a 64-bit unsigned integer.
///
/// The bits are indexed from a1 to h8, with a1 being the least significant bit
/// (rightmost bit) and h8 being the most significant bit (leftmost bit).
///
/// # Examples
///
/// ```
/// # use chess_engine_core::*;
/// let bitboard = bitboard! {
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     X X X X X X X X
///     . . . . . . . .
/// };
/// println!("{}", bitboard);
/// assert_eq!(bitboard, BitBoard(0x000000000000FF00));
/// ```
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitBoard(pub u64);

impl BitBoard {
    /// An empty `BitBoard`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = BitBoard::EMPTY;
    /// assert_eq!(bitboard, bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// });
    /// ```
    pub const EMPTY: Self = Self(0);

    /// Sets a square on a `BitBoard`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = BitBoard::EMPTY.set_square(Square::B2);
    /// assert_eq!(bitboard, bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . X . . . . . .
    ///     . . . . . . . .
    /// });
    /// ```
    pub const fn set_square(&self, square: Square) -> Self {
        self.set_bit(square.bitboard().0)
    }

    /// Sets a bit on a `BitBoard`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = BitBoard::EMPTY.set_bit(0x000000000000FF00);
    /// assert_eq!(bitboard, BitBoard(0x000000000000FF00));
    /// ```
    pub const fn set_bit(&self, bit: u64) -> Self {
        Self(self.0 | bit)
    }

    /// Unsets a square on a `BitBoard`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . X . . . . . .
    ///     . . . . . . . .
    /// };
    ///
    /// assert_eq!(bitboard.unset_square(Square::B2), BitBoard::EMPTY);
    /// assert_eq!(bitboard.unset_square(Square::A1), bitboard);
    /// ```
    pub const fn unset_square(&self, square: Square) -> Self {
        self.unset_bit(square.bitboard().0)
    }

    /// Unsets a bit on a `BitBoard`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = BitBoard(0x000000000000FF00);
    /// let result = bitboard.unset_bit(0x000000000000FF00);
    /// assert_eq!(result, BitBoard::EMPTY);
    /// ```
    pub const fn unset_bit(&self, bit: u64) -> Self {
        Self(self.0 & !bit)
    }

    /// Returns `true` if a square is set on a `BitBoard`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . X . . . . . .
    ///     . . . . . . . .
    /// };
    ///
    /// assert!(!bitboard.is_get_square(Square::A1));
    /// assert!(bitboard.is_get_square(Square::B2));
    /// ```
    pub const fn is_get_square(&self, square: Square) -> bool {
        self.is_get_bit(square.bitboard().0)
    }

    /// Returns `true` if a bit is set on a `BitBoard`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = BitBoard(0x000000000000FFFF);
    /// assert!(bitboard.is_get_bit(0x000000000000FF00));
    /// assert!(bitboard.is_get_bit(0x00000000000000FF));
    /// ```
    pub const fn is_get_bit(&self, bit: u64) -> bool {
        self.0 & bit != 0
    }

    /// Shifts the bits of a `BitBoard` up by one rank.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . X . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// };
    /// assert_eq!(bitboard.up(), bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . X . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// });
    pub const fn up(self) -> Self {
        Self(self.0 << 8)
    }

    /// Shifts the bits of a `BitBoard` down by one rank.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . X . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// };
    /// assert_eq!(bitboard.down(), bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . X . . .
    ///     . . . . . . . .
    /// });
    pub const fn down(self) -> Self {
        Self(self.0 >> 8)
    }

    /// Shifts the bits of a `BitBoard` to the right by one file.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . X . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// };
    /// assert_eq!(bitboard.right(), bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . X . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// });
    pub const fn right(self) -> Self {
        const NOT_FILE_A: u64 = bitboard! {
            . X X X X X X X
            . X X X X X X X
            . X X X X X X X
            . X X X X X X X
            . X X X X X X X
            . X X X X X X X
            . X X X X X X X
            . X X X X X X X
        }
        .0;

        Self(self.0 << 1 & NOT_FILE_A)
    }

    /// Shifts the bits of a `BitBoard` to the left by one file.
    ///
    /// # Examples
    ///
    /// /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . X . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// };
    /// assert_eq!(bitboard.left(), bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . X . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// });
    pub const fn left(self) -> Self {
        const NOT_FILE_H: u64 = bitboard! {
            X X X X X X X .
            X X X X X X X .
            X X X X X X X .
            X X X X X X X .
            X X X X X X X .
            X X X X X X X .
            X X X X X X X .
            X X X X X X X .
        }
        .0;
        Self(self.0 >> 1 & NOT_FILE_H)
    }

    /// Returns the least significant square of a `BitBoard`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . X X X X . .
    ///     . . . . X . . .
    ///     . . . X . . . .
    ///     . . X . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// };
    /// assert_eq!(bitboard.least_significant_square(), Some(Square::C3));
    /// ```
    pub const fn least_significant_square(self) -> Option<Square> {
        if self.0 == 0 {
            return None;
        }

        Some(Square::new(self.0.trailing_zeros() as usize))
    }

    /// Returns the number of set bits in a `BitBoard`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let bitboard = bitboard! {
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . X X X X . .
    ///     . . . . X . . .
    ///     . . . X . . . .
    ///     . . X . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// };
    /// assert_eq!(bitboard.len(), 7);
    /// ```
    pub const fn len(self) -> usize {
        self.0.count_ones() as usize
    }
}

macro_rules! impl_ops {
    ($($trait:ident, $fn:ident;)*) => {$(
        impl $trait for BitBoard {
            type Output = Self;

            #[inline(always)]
            fn $fn(self, rhs: Self) -> Self::Output {
                Self($trait::$fn(self.0, rhs.0))
            }
        }

        impl $trait<u64> for BitBoard {
            type Output = Self;

            #[inline(always)]
            fn $fn(self, rhs: u64) -> Self::Output {
                Self($trait::$fn(self.0, rhs))
            }
        }

        impl $trait<Square> for BitBoard {
            type Output = Self;

            #[inline(always)]
            fn $fn(self, rhs: Square) -> Self::Output {
                Self($trait::$fn(self.0, BitBoard::from(rhs).0))
            }
        }

        impl $trait<File> for BitBoard {
            type Output = Self;

            #[inline(always)]
            fn $fn(self, rhs: File) -> Self::Output {
                Self($trait::$fn(self.0, BitBoard::from(rhs).0))
            }
        }

        impl $trait<Rank> for BitBoard {
            type Output = Self;

            #[inline(always)]
            fn $fn(self, rhs: Rank) -> Self::Output {
                Self($trait::$fn(self.0, BitBoard::from(rhs).0))
            }
        }
    )*};
}

macro_rules! impl_assign_ops {
    ($($trait:ident, $fn:ident;)*) => {$(
        impl $trait for BitBoard {
            #[inline(always)]
            fn $fn(&mut self, rhs: Self) {
                $trait::$fn(&mut self.0, rhs.0)
            }
        }

        impl $trait<u64> for BitBoard {
            #[inline(always)]
            fn $fn(&mut self, rhs: u64) {
                $trait::$fn(&mut self.0, rhs)
            }
        }

        impl $trait<Square> for BitBoard {
            #[inline(always)]
            fn $fn(&mut self, rhs: Square) {
                $trait::$fn(&mut self.0, BitBoard::from(rhs).0)
            }
        }

        impl $trait<File> for BitBoard {
            #[inline(always)]
            fn $fn(&mut self, rhs: File) {
                $trait::$fn(&mut self.0, BitBoard::from(rhs).0)
            }
        }

        impl $trait<Rank> for BitBoard {
            #[inline(always)]
            fn $fn(&mut self, rhs: Rank) {
                $trait::$fn(&mut self.0, BitBoard::from(rhs).0)
            }
        }
    )*};
}

impl_ops! {
    BitAnd, bitand;
    BitOr, bitor;
    BitXor, bitxor;
}

impl_assign_ops! {
    BitAndAssign, bitand_assign;
    BitOrAssign, bitor_assign;
    BitXorAssign, bitxor_assign;
}

impl PartialEq<u64> for BitBoard {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u64> for BitBoard {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

/// Performs a bitwise NOT operation on a `BitBoard`.
///
/// # Examples
///
/// ```
/// # use chess_engine_core::*;
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
/// # Examples
///
/// ```
/// # use chess_engine_core::*;
/// let bitboard: BitBoard = Square::C2.into();
/// assert_eq!(bitboard, bitboard! {
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . X . . . . .
///     . . . . . . . .
/// });
/// ```
impl From<Square> for BitBoard {
    fn from(square: Square) -> Self {
        square.bitboard()
    }
}

/// Converts a [`File`] to a [`BitBoard`].
///
/// # Examples
///
/// ```
/// # use chess_engine_core::*;
/// let bitboard: BitBoard = File::C.into();
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
impl From<File> for BitBoard {
    fn from(file: File) -> Self {
        file.bitboard()
    }
}

/// Converts a [`Rank`] to a [`BitBoard`].
///
/// # Examples
///
/// ```
/// # use chess_engine_core::*;
/// let bitboard: BitBoard = Rank::Three.into();
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
impl From<Rank> for BitBoard {
    fn from(rank: Rank) -> Self {
        rank.bitboard()
    }
}

/// Formats a `BitBoard` as a string.
///
/// The string is formatted as a 16-character hexadecimal number.
///
/// # Examples
///
/// ```
/// # use chess_engine_core::*;
/// let bitboard = BitBoard(0x000000000000FF00);
/// println!("{}", bitboard);
/// ```
///
/// ```textplain
/// 000000000000FF00
/// ```
impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:016X}", self.0)
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
/// # use chess_engine_core::*;
/// let bitboard = BitBoard(0x000000000000FF00);
/// println!("{:?}", bitboard);
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
impl Debug for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("\n");

        for rank in (0..Rank::LEN).rev() {
            let rank = Rank::new(rank);
            s.push_str(&format!("  {} ", rank));

            for file in 0..File::LEN {
                let file = File::new(file);

                if BitBoard::from(Square::with_file_rank(file, rank)) & *self != 0 {
                    s.push_str("X ");
                } else {
                    s.push_str(". ");
                }
            }

            s.push_str("\n");
        }

        s.push_str("\n    a b c d e f g h\n\n");

        s.push_str(&format!("   {:016X}", self.0));

        write!(f, "{}", s)
    }
}

/// An iterator over the set bits of a `BitBoard`.
///
/// # Examples
///
/// ```
/// # use chess_engine_core::*;
/// let bitboard = bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     X X X X X X X X
///     . . . . . . . .
/// };
/// let mut iter = bitboard.into_iter();
/// assert_eq!(iter.next(), Some(Square::A2));
/// assert_eq!(iter.next(), Some(Square::B2));
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
        let square = self.bitboard.least_significant_square()?;

        self.bitboard = self.bitboard.unset_square(square);

        Some(square)
    }
}

/// Converts a `BitBoard` into an iterator over its set bits.
///
/// # Examples
///
/// ```no_run
/// # use chess_engine_core::*;
/// let bitboard = bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . X X X X . .
///     . . . . X . . .
///     . . . X . . . .
///     . . X . . . . .
///     . . . . . . . .
///     . . . . . . . .
/// };
/// let mut iter = bitboard.into_iter();
/// assert_eq!(iter.next(), Some(Square::C3));
/// assert_eq!(iter.next(), Some(Square::D4));
/// ```
impl IntoIterator for BitBoard {
    type Item = Square;
    type IntoIter = BitBoardIter;

    fn into_iter(self) -> Self::IntoIter {
        BitBoardIter { bitboard: self }
    }
}
