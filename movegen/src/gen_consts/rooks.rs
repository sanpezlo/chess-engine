use chess_engine_core::{BitBoard, File, Rank, Square};

use crate::gen_consts::magic::mask_blockers;

/// Returns the mask for the relevant rook blockers for a square.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let relevant_rook_blockers = mask_relevant_rook_blockers(Square::E4);
/// assert_eq!(relevant_rook_blockers, bitboard!{
///     . . . . . . . .
///     . . . . X . . .
///     . . . . X . . .
///     . . . . X . . .
///     . X X X . X X .
///     . . . . X . . .
///     . . . . X . . .
///     . . . . . . . .
/// });
///
/// let relevant_rook_blockers = mask_relevant_rook_blockers(Square::A1);
/// assert_eq!(relevant_rook_blockers, bitboard!{
///     . . . . . . . .
///     X . . . . . . .
///     X . . . . . . .
///     X . . . . . . .
///     X . . . . . . .
///     X . . . . . . .
///     X . . . . . . .
///     . X X X X X X .
/// });
/// ```
pub const fn mask_relevant_rook_blockers(square: Square) -> BitBoard {
    let mut mask_rook = mask_rook_attacks(square, BitBoard::EMPTY).0;

    let (rank, file) = (square.rank(), square.file());

    if rank as usize != Rank::One as usize {
        mask_rook &= !Rank::One.bitboard().0;
    }

    if rank as usize != Rank::Eight as usize {
        mask_rook &= !Rank::Eight.bitboard().0;
    }

    if file as usize != File::A as usize {
        mask_rook &= !File::A.bitboard().0;
    }

    if file as usize != File::H as usize {
        mask_rook &= !File::H.bitboard().0;
    }

    BitBoard(mask_rook)
}

/// Returns the mask for the rook attacks for a square with blockers.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let attacks = mask_rook_attacks(Square::E4, bitboard!{
///     . . . . . . . .
///     . . . . X . . .
///     . . . . . . . .
///     . . . . X . . .
///     . X . . . . . X
///     . . . . . . . .
///     . . . . X . . .
///     . . . . X . . .
/// });
///
/// assert_eq!(attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . X . . .
///     . X X X . X X X
///     . . . . X . . .
///     . . . . X . . .
///     . . . . . . . .
/// });
/// ```
pub const fn mask_rook_attacks(square: Square, blockers: BitBoard) -> BitBoard {
    let square = BitBoard(1u64 << square as usize);

    let mut attacks = BitBoard::EMPTY;

    let mut up = square.up();
    while up.0 != BitBoard::EMPTY.0 {
        attacks = attacks.set_bit(up.0);

        if blockers.is_get_bit(up.0) {
            break;
        }

        up = up.up();
    }

    let mut right = square.right();
    while right.0 != BitBoard::EMPTY.0 {
        attacks = attacks.set_bit(right.0);

        if blockers.is_get_bit(right.0) {
            break;
        }

        right = right.right();
    }

    let mut down = square.down();
    while down.0 != BitBoard::EMPTY.0 {
        attacks = attacks.set_bit(down.0);

        if blockers.is_get_bit(down.0) {
            break;
        }

        down = down.down();
    }

    let mut left = square.left();
    while left.0 != BitBoard::EMPTY.0 {
        attacks = attacks.set_bit(left.0);

        if blockers.is_get_bit(left.0) {
            break;
        }

        left = left.left();
    }

    attacks
}

/// Writes to a file
///
/// - the precomputed rook attacks for all squares and blockers.
/// - the relevant rook blockers count for all squares.
pub fn write(f: &mut std::fs::File, magic_numbers: &[u64; Square::LEN]) {
    use std::io::Write;

    let mut blockers_count = [0; Square::LEN];

    // relevant rook blockers

    write!(f, "/// Relevant rook blockers for all squares\n").unwrap();

    write!(
        f,
        "pub const RELEVANT_ROOK_BLOCKERS: [BitBoard; {}] = [\n",
        Square::LEN
    )
    .unwrap();

    for square in Square::ALL {
        write!(
            f,
            "\tBitBoard({}),\n",
            mask_relevant_rook_blockers(square).0
        )
        .unwrap();
    }

    write!(f, "];\n").unwrap();

    write!(f, "/// Relevant bishop blockers count for all squares\n").unwrap();

    write!(
        f,
        "pub const RELEVANT_ROOK_BLOCKERS_COUNT: [u8; {}] = [\n",
        Square::LEN
    )
    .unwrap();

    for square in Square::ALL {
        let count = mask_relevant_rook_blockers(square).0.count_ones() as u8;

        blockers_count[square as usize] = count;
        write!(f, "\t{},\n", count).unwrap();
    }

    write!(f, "];\n").unwrap();

    // precomputed rook attacks

    let max_blockers = 1 << blockers_count.iter().max().unwrap();

    let mut attacks: Vec<Vec<BitBoard>> = vec![vec![BitBoard::EMPTY; max_blockers]; Square::LEN];

    for square in Square::ALL {
        for blockers_pattern in 0..max_blockers {
            let blockers =
                mask_blockers(blockers_pattern as u64, mask_relevant_rook_blockers(square));

            let magic_index = (blockers.0.wrapping_mul(magic_numbers[square as usize]))
                >> (64 - blockers_count[square as usize]);

            attacks[square as usize][magic_index as usize] = mask_rook_attacks(square, blockers);
        }
    }

    write!(f, "/// Precomputed rook attacks for all squares\n").unwrap();

    write!(
        f,
        "pub const ROOK_ATTACKS: [[BitBoard; {}]; {}] = [\n",
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
