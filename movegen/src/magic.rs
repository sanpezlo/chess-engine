use chess_engine_core::{BitBoard, SlidingPiece, Square};

include!(concat!(env!("OUT_DIR"), "/magic_gen.rs"));

/// Returns the bishop attacks for a square with blockers.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let attacks = get_bishop_attacks(Square::E4, bitboard!{
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
pub fn get_bishop_attacks(square: Square, blockers: BitBoard) -> BitBoard {
    let magic_index = blockers.0 & RELEVANT_BISHOP_BLOCKERS[square as usize].0;
    println!("{:?}", BitBoard(magic_index));
    println!("{:?}", RELEVANT_BISHOP_BLOCKERS[square as usize]);
    println!(
        "{:?}",
        mask_blockers(0b1001011, RELEVANT_BISHOP_BLOCKERS[square as usize])
    );
    println!("{:?}", BISHOP_ATTACKS[square as usize][0b1001011 as usize]);
    let magic_index = magic_index
        .wrapping_mul(MAGIC_NUMBERS[SlidingPiece::Bishop as usize][square as usize])
        >> (64 - RELEVANT_BISHOP_BLOCKERS_COUNT[square as usize]);

    BISHOP_ATTACKS[square as usize][magic_index as usize]
}

/// Returns the rook attacks for a square with blockers.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let attacks = get_rook_attacks(Square::E4, bitboard!{
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
pub fn get_rook_attacks(square: Square, blockers: BitBoard) -> BitBoard {
    let magic_index = blockers.0 & RELEVANT_ROOK_BLOCKERS[square as usize].0;
    println!("{:?}", BitBoard(magic_index));
    println!("{:?}", RELEVANT_ROOK_BLOCKERS[square as usize]);
    println!(
        "{:?}",
        mask_blockers(0b1001011, RELEVANT_ROOK_BLOCKERS[square as usize])
    );
    println!("{:?}", ROOK_ATTACKS[square as usize][0b1001011 as usize]);
    let magic_index = magic_index
        .wrapping_mul(MAGIC_NUMBERS[SlidingPiece::Rook as usize][square as usize])
        >> (64 - RELEVANT_ROOK_BLOCKERS_COUNT[square as usize]);

    ROOK_ATTACKS[square as usize][magic_index as usize]
}

const fn mask_blockers(pattern: u64, blockers: BitBoard) -> BitBoard {
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
