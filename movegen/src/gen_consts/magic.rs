use chess_engine_core::{random_magic_number, BitBoard, SlidingPiece, Square};

use crate::gen_consts::{bishops, rooks};

use super::{
    bishops::{mask_bishop_attacks, mask_relevant_bishop_blockers},
    rooks::{mask_relevant_rook_blockers, mask_rook_attacks},
};

/// Returns the blockers mask for a given pattern and blockers.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let blockers = bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . X X X X . .
///     . . . . X . . .
///     . . . X . . . .
///     . . X . . . . .
///     . . . . . . . .
///     . . . . . . . .
/// };
///
/// assert_eq!(mask_blockers(0b1001111, blockers), bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . X . . X . .
///     . . . . X . . .
///     . . . X . . . .
///     . . X . . . . .
///     . . . . . . . .
///     . . . . . . . .
/// });
///
/// assert_eq!(mask_blockers(0b1011001, blockers), bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . X X . X . .
///     . . . . . . . .
///     . . . . . . . .
///     . . X . . . . .
///     . . . . . . . .
///     . . . . . . . .
/// });
/// ```
pub const fn mask_blockers(pattern: u64, blockers: BitBoard) -> BitBoard {
    let mut mask = BitBoard::EMPTY;
    let mut blockers = blockers;

    let mut i = 0;

    while let Some(square) = blockers.least_significant_square() {
        blockers = blockers.unset_square(square);

        if pattern & (1 << i) != 0 {
            mask = mask.set_square(square);
        }

        i += 1;
    }

    mask
}

/// Finds the magic number for a given square and sliding piece.
fn magic_number(square: Square, sliding_piece: SlidingPiece) -> u64 {
    let relevant_bits: u8 = match sliding_piece {
        SlidingPiece::Bishop => mask_relevant_bishop_blockers(square).0.count_ones() as u8,
        SlidingPiece::Rook => mask_relevant_rook_blockers(square).0.count_ones() as u8,
    };

    let mut blockers = [BitBoard::EMPTY; 4096];
    let mut attacks = [BitBoard::EMPTY; 4096];
    let mut used_attacks: [BitBoard; 4096];

    let relevant_blockers = match sliding_piece {
        SlidingPiece::Bishop => mask_relevant_bishop_blockers(square),
        SlidingPiece::Rook => mask_relevant_rook_blockers(square),
    };

    let blockers_index = 1 << relevant_bits;

    let mut index = 0;
    while index < blockers_index {
        blockers[index] = mask_blockers(index as u64, relevant_blockers);

        attacks[index] = match sliding_piece {
            SlidingPiece::Bishop => mask_bishop_attacks(square, blockers[index]),
            SlidingPiece::Rook => mask_rook_attacks(square, blockers[index]),
        };

        index += 1;
    }

    loop {
        let magic_number = random_magic_number();

        if (relevant_blockers.0.wrapping_mul(magic_number) & 0xFF00000000000000).count_ones() < 6 {
            continue;
        }

        used_attacks = [BitBoard::EMPTY; 4096];

        let mut index = 0;
        let mut fail = false;

        while index < blockers_index && !fail {
            let magic_index =
                (blockers[index].0.wrapping_mul(magic_number)) >> (64 - relevant_bits);

            if used_attacks[magic_index as usize].0 == BitBoard::EMPTY.0 {
                used_attacks[magic_index as usize] = attacks[index];
            } else if used_attacks[magic_index as usize].0 != attacks[index].0 {
                fail = true;
            }

            index += 1;
        }

        if !fail {
            return magic_number;
        }
    }
}

/// Writes to a file the magic numbers for all squares and sliding pieces.
pub fn write(f: &mut std::fs::File) {
    use std::io::Write;

    let mut magic_numbers = [[0u64; Square::LEN]; SlidingPiece::LEN];

    write!(f, "/// Magic numbers for all squares and sliding pieces\n").unwrap();

    write!(
        f,
        "pub const MAGIC_NUMBERS: [[u64; {}]; {}] = [\n",
        Square::LEN,
        SlidingPiece::LEN
    )
    .unwrap();

    for sliding_piece in SlidingPiece::ALL {
        write!(f, "\t[\n").unwrap();
        for square in Square::ALL {
            let magic_number = magic_number(square, sliding_piece);

            magic_numbers[sliding_piece as usize][square as usize] = magic_number;
            write!(f, "\t\t{},\n", magic_number).unwrap();
        }

        write!(f, "\t],\n").unwrap();
    }

    write!(f, "];\n").unwrap();

    bishops::write(f, &magic_numbers[SlidingPiece::Bishop as usize]);
    rooks::write(f, &magic_numbers[SlidingPiece::Rook as usize]);
}
