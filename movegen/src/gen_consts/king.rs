use chess_engine_core::{BitBoard, Square};

/// Returns the mask for the king attacks for a square.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let king_attacks = mask_king_attacks(Square::E4);
///
/// assert_eq!(king_attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . X X X . .
///     . . . X . X . .
///     . . . X X X . .
///     . . . . . . . .
///     . . . . . . . .
/// });
/// ```
pub const fn mask_king_attacks(square: Square) -> BitBoard {
    let square = BitBoard(1u64 << square as usize);

    let mut attacks = BitBoard::EMPTY;

    attacks = attacks.set_bit(square.up().0);
    attacks = attacks.set_bit(square.up().right().0);
    attacks = attacks.set_bit(square.right().0);
    attacks = attacks.set_bit(square.down().right().0);
    attacks = attacks.set_bit(square.down().0);
    attacks = attacks.set_bit(square.down().left().0);
    attacks = attacks.set_bit(square.left().0);
    attacks = attacks.set_bit(square.up().left().0);

    attacks
}

/// Writes to a file the precomputed king attacks for all squares.
pub fn write(f: &mut std::fs::File) {
    use std::io::Write;

    write!(f, "/// Precomputed king attacks for all squares\n").unwrap();

    write!(
        f,
        "pub const KING_ATTACKS: [BitBoard; {}] = [\n",
        Square::LEN
    )
    .unwrap();

    for square in Square::ALL {
        write!(f, "\tBitBoard({}),\n", mask_king_attacks(square).0).unwrap();
    }

    write!(f, "];\n").unwrap();
}
