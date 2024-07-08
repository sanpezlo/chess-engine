use chess_engine_core::{bitboard, BitBoard, Square};

use crate::gen_consts::magic::mask_blockers;

/// Returns the mask of the relevant bishop blockers for a square.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let relevant_bishop_blockers = mask_relevant_bishop_blockers(Square::E4);
///
/// assert_eq!(relevant_bishop_blockers, bitboard!{
///     . . . . . . . .
///     . X . . . . . .
///     . . X . . . X .
///     . . . X . X . .
///     . . . . . . . .
///     . . . X . X . .
///     . . X . . . X .
///     . . . . . . . .
/// });
/// ```
pub const fn mask_relevant_bishop_blockers(square: Square) -> BitBoard {
    const RELEVANT_BLOCKERS: BitBoard = bitboard! {
        . . . . . . . .
        . X X X X X X .
        . X X X X X X .
        . X X X X X X .
        . X X X X X X .
        . X X X X X X .
        . X X X X X X .
        . . . . . . . .
    };

    BitBoard(mask_bishop_attacks(square, BitBoard::EMPTY).0 & RELEVANT_BLOCKERS.0)
}

/// Returns the mask for the bishop attacks for a square with blockers.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let attacks = mask_bishop_attacks(Square::E4, bitboard!{
///     X . . . . . . .
///     . . . . . . . X
///     . . X . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . X . .
///     . . X . . . X .
///     . . . . . . . .
/// });
///
/// assert_eq!(attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . X
///     . . X . . . X .
///     . . . X . X . .
///     . . . . . . . .
///     . . . X . X . .
///     . . X . . . . .
///     . . . . . . . .
/// });
/// ```
pub const fn mask_bishop_attacks(square: Square, blockers: BitBoard) -> BitBoard {
    let square = BitBoard(1u64 << square as usize);

    let mut attacks = BitBoard::EMPTY;

    let mut up_right = square.up().right();
    while up_right.0 != BitBoard::EMPTY.0 {
        attacks = attacks.set_bit(up_right.0);

        if blockers.is_get_bit(up_right.0) {
            break;
        }

        up_right = up_right.up().right();
    }

    let mut up_left = square.up().left();
    while up_left.0 != BitBoard::EMPTY.0 {
        attacks = attacks.set_bit(up_left.0);

        if blockers.is_get_bit(up_left.0) {
            break;
        }

        up_left = up_left.up().left();
    }

    let mut down_right = square.down().right();
    while down_right.0 != BitBoard::EMPTY.0 {
        attacks = attacks.set_bit(down_right.0);

        if blockers.is_get_bit(down_right.0) {
            break;
        }

        down_right = down_right.down().right();
    }

    let mut down_left = square.down().left();
    while down_left.0 != BitBoard::EMPTY.0 {
        attacks = attacks.set_bit(down_left.0);

        if blockers.is_get_bit(down_left.0) {
            break;
        }

        down_left = down_left.down().left();
    }

    attacks
}

/// Writes to a file
///
/// - the precomputed bishop attacks for all squares and blockers.
/// - the relevant bishop blockers count for all squares.
pub fn write(f: &mut std::fs::File, magic_numbers: &[u64; Square::LEN]) {
    use std::io::Write;

    let mut blockers_count = [0; Square::LEN];

    // relevant bishop blockers

    write!(
        f,
        "/// relevant bishop blockers for all squares\n"
    )
    .unwrap();

    write!(
        f,
        "pub const RELEVANT_BISHOP_BLOCKERS: [BitBoard; {}] = [\n",
        Square::LEN
    )
    .unwrap();

    for square in Square::ALL {
        write!(
            f,
            "\tBitBoard({}),\n",
            mask_relevant_bishop_blockers(square).0
        )
        .unwrap();
    }

    write!(f, "];\n").unwrap();

    write!(
        f,
        "/// relevant bishop blockers count for all squares\n"
    )
    .unwrap();

    write!(
        f,
        "pub const RELEVANT_BISHOP_BLOCKERS_COUNT: [u8; {}] = [\n",
        Square::LEN
    )
    .unwrap();

    for square in Square::ALL {
        let count = mask_relevant_bishop_blockers(square).0.count_ones() as u8;
        blockers_count[square as usize] = count;
        write!(f, "\t{},\n", count).unwrap();
    }

    write!(f, "];\n").unwrap();

    // precomputed bishop attacks

    let max_blockers = 1 << blockers_count.iter().max().unwrap();

    let mut attacks: Vec<Vec<BitBoard>> = vec![vec![BitBoard::EMPTY; max_blockers]; Square::LEN];

    for square in Square::ALL {
        for blockers_pattern in 0..max_blockers {
            let blockers = mask_blockers(
                blockers_pattern as u64,
                mask_relevant_bishop_blockers(square),
            );

            let magic_index = (blockers.0.wrapping_mul(magic_numbers[square as usize]))
                >> (64 - blockers_count[square as usize]);

            attacks[square as usize][magic_index as usize] = mask_bishop_attacks(square, blockers);
        }
    }

    write!(
        f,
        "/// precomputed bishop attacks for all squares and blockers\n"
    )
    .unwrap();

    write!(
        f,
        "pub const BISHOP_ATTACKS: [[BitBoard; {}]; {}] = [\n",
        max_blockers,
        Square::LEN
    )
    .unwrap();

    for square in Square::ALL {
        write!(f, "\t[\n").unwrap();

        for blockers_pattern in 0..max_blockers {
            write!(
                f,
                "\t\tBitBoard({}),\n",
                attacks[square as usize][blockers_pattern].0
            )
            .unwrap();
        }

        write!(f, "\t],\n").unwrap();
    }

    write!(f, "];\n").unwrap();
}
