use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn new(bits: u64) -> Self {
        Self(bits)
    }
}

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

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("\n");

        const LAST_BIT: u64 = 63;
        const LAST_RANK: u64 = 8;

        for rank in 0..8 {
            s.push_str(&format!("  {} ", LAST_RANK - rank));

            for file in (0..8).rev() {
                if self.0 & (1u64 << (LAST_BIT - (rank * 8) - file)) != 0 {
                    s.push_str(" X");
                } else {
                    s.push_str(" .");
                }
            }

            s.push_str("\n");
        }

        s.push_str("\n     a b c d e f g h\n\n");

        s.push_str(&format!("   {:016X}\n", self.0));

        write!(f, "{}", s)
    }
}
