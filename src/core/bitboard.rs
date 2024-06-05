use crate::{File, Rank, Square};
use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct BitBoard(pub u64);

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl PartialEq for BitBoard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl From<Square> for BitBoard {
    fn from(square: Square) -> Self {
        Self(1u64 << square.0)
    }
}

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
