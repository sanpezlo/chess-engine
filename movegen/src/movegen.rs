use chess_engine_core::{
    bitboard, random_magic_number, BitBoard, Color, File, Rank, SlidingPiece, Square,
};

/// The pawn attacks for each color and square.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let white_pawn_attacks = PAWNS_ATTACKS[Color::White as usize][Square::E4 as usize];
/// let black_pawn_attacks = PAWNS_ATTACKS[Color::Black as usize][Square::E4 as usize];
///
/// assert_eq!(white_pawn_attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . X . X . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
/// });
/// assert_eq!(black_pawn_attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . X . X . .
///     . . . . . . . .
///     . . . . . . . .
/// });
/// ```
pub const PAWNS_ATTACKS: [[BitBoard; Square::LEN]; Color::LEN] = {
    let mut attacks = [[BitBoard::EMPTY; Square::LEN]; Color::LEN];

    let mut index = 0;
    while index < Square::LEN {
        attacks[Color::White as usize][index] = mask_pawn_attacks(Color::White, Square::new(index));
        attacks[Color::Black as usize][index] = mask_pawn_attacks(Color::Black, Square::new(index));

        index += 1;
    }

    attacks
};

/// The knight attacks for each square.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let knight_attacks = KNIGHTS_ATTACKS[Square::E4 as usize];
///
/// assert_eq!(knight_attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . . X . X . .
///     . . X . . . X .
///     . . . . . . . .
///     . . X . . . X .
///     . . . X . X . .
///     . . . . . . . .
/// });
pub const KNIGHTS_ATTACKS: [BitBoard; Square::LEN] = {
    let mut attacks = [BitBoard::EMPTY; Square::LEN];

    let mut index = 0;
    while index < Square::LEN {
        attacks[index] = mask_knight_attacks(Square::new(index));

        index += 1;
    }

    attacks
};

/// The king attacks for each square.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let king_attacks = KINGS_ATTACKS[Square::E4 as usize];
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
pub const KINGS_ATTACKS: [BitBoard; Square::LEN] = {
    let mut attacks = [BitBoard::EMPTY; Square::LEN];

    let mut index = 0;
    while index < Square::LEN {
        attacks[index] = mask_king_attacks(Square::new(index));

        index += 1;
    }

    attacks
};

/// The relevant bishop occupancy for each square.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let bishop_occupancy = RELEVANT_BISHOP_OCCUPANCY[Square::E4 as usize];
///
/// assert_eq!(bishop_occupancy, bitboard!{
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
pub const RELEVANT_BISHOP_OCCUPANCY: [BitBoard; Square::LEN] = {
    let mut occupancy = [BitBoard::EMPTY; Square::LEN];

    let mut index = 0;
    while index < Square::LEN {
        occupancy[index] = mask_bishop_occupancy(Square::new(index));

        index += 1;
    }

    occupancy
};

/// The relevant rook occupancy for each square.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let rook_occupancy = RELEVANT_ROOK_OCCUPANCY[Square::E4 as usize];
/// assert_eq!(rook_occupancy, bitboard!{
///     . . . . . . . .
///     . . . . X . . .
///     . . . . X . . .
///     . . . . X . . .
///     . X X X . X X .
///     . . . . X . . .
///     . . . . X . . .
///     . . . . . . . .
/// });
/// ```
pub const RELEVANT_ROOK_OCCUPANCY: [BitBoard; Square::LEN] = {
    let mut occupancy = [BitBoard::EMPTY; Square::LEN];

    let mut index = 0;
    while index < Square::LEN {
        occupancy[index] = mask_rook_occupancy(Square::new(index));

        index += 1;
    }

    occupancy
};

/// The number of relevant bishop occupancy for each square.
///
/// ```textplain
/// 6 5 5 5 5 5 5 6
/// 5 5 5 5 5 5 5 5
/// 5 5 7 7 7 7 5 5
/// 5 5 7 9 9 7 5 5
/// 5 5 7 9 9 7 5 5
/// 5 5 7 7 7 7 5 5
/// 5 5 5 5 5 5 5 5
/// 6 5 5 5 5 5 5 6
/// ```
pub const RELEVANT_BISHOP_OCCUPANCY_COUNT: [u8; Square::LEN] = {
    let mut count = [0; Square::LEN];

    let mut index = 0;
    while index < Square::LEN {
        count[index] = RELEVANT_BISHOP_OCCUPANCY[index].0.count_ones() as u8;

        index += 1;
    }

    count
};

/// The number of relevant rook occupancy for each square.
///
/// ```textplain
/// 12 11 11 11 11 11 11 12
/// 11 10 10 10 10 10 10 11
/// 11 10 10 10 10 10 10 11
/// 11 10 10 10 10 10 10 11
/// 11 10 10 10 10 10 10 11
/// 11 10 10 10 10 10 10 11
/// 11 10 10 10 10 10 10 11
/// 12 11 11 11 11 11 11 12
/// ```
pub const RELEVANT_ROOK_OCCUPANCY_COUNT: [u8; Square::LEN] = {
    let mut count = [0; Square::LEN];

    let mut index = 0;
    while index < Square::LEN {
        count[index] = RELEVANT_ROOK_OCCUPANCY[index].0.count_ones() as u8;

        index += 1;
    }

    count
};

// pub const ROOK_MAGIC_NUMBERS: [u64; Square::LEN] = {
//     let mut magic_numbers = [0; Square::LEN];

//     let mut index = 0;
//     while index < Square::LEN {
//         magic_numbers[index] = magic_number(Square::new(index), SlidingPiece::Rook);

//         index += 1;
//     }

//     magic_numbers
// };

// pub const BISHOP_MAGIC_NUMBERS: [u64; Square::LEN] = {
//     let mut magic_numbers = [0; Square::LEN];

//     let mut index = 0;
//     while index < Square::LEN {
//         magic_numbers[index] = magic_number(Square::new(index), SlidingPiece::Rook);

//         index += 1;
//     }

//     magic_numbers
// };

/// Returns the mask for the pawn attacks for a square.
const fn mask_pawn_attacks(color: Color, square: Square) -> BitBoard {
    let square = BitBoard(1u64 << square as usize);

    let mut attacks = BitBoard::EMPTY;

    match color {
        Color::White => {
            attacks = attacks.set_bit(square.up().right().0);
            attacks = attacks.set_bit(square.up().left().0);
        }
        Color::Black => {
            attacks = attacks.set_bit(square.down().right().0);
            attacks = attacks.set_bit(square.down().left().0);
        }
    }

    attacks
}

/// Returns the mask for the knight attacks for a square.
const fn mask_knight_attacks(square: Square) -> BitBoard {
    let square = BitBoard(1u64 << square as usize);

    let mut attacks = BitBoard::EMPTY;

    attacks = attacks.set_bit(square.up().up().right().0);
    attacks = attacks.set_bit(square.up().up().left().0);
    attacks = attacks.set_bit(square.right().right().up().0);
    attacks = attacks.set_bit(square.right().right().down().0);
    attacks = attacks.set_bit(square.down().down().right().0);
    attacks = attacks.set_bit(square.down().down().left().0);
    attacks = attacks.set_bit(square.left().left().up().0);
    attacks = attacks.set_bit(square.left().left().down().0);

    attacks
}

/// Returns the mask for the king attacks for a square.
const fn mask_king_attacks(square: Square) -> BitBoard {
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

/// Returns the mask for the bishop occupancy for a square.
const fn mask_bishop_occupancy(square: Square) -> BitBoard {
    const RELEVANT_OCCUPANCY: BitBoard = bitboard! {
        . . . . . . . .
        . X X X X X X .
        . X X X X X X .
        . X X X X X X .
        . X X X X X X .
        . X X X X X X .
        . X X X X X X .
        . . . . . . . .
    };

    BitBoard(mask_bishop_attacks(square, BitBoard::EMPTY).0 & RELEVANT_OCCUPANCY.0)
}

/// Returns the mask for the bishop attacks for a square with blockers.
const fn mask_bishop_attacks(square: Square, blockers: BitBoard) -> BitBoard {
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

/// Returns the mask for the rook occupancy for a square.
const fn mask_rook_occupancy(square: Square) -> BitBoard {
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
const fn mask_rook_attacks(square: Square, blockers: BitBoard) -> BitBoard {
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

/// Returns the occupancy mask for a given pattern and mask.
const fn mask_occupancy(pattern: u64, mask: BitBoard) -> BitBoard {
    let mut occupancy = BitBoard::EMPTY;
    let mut mask = mask;

    let mut i = 0;

    while let Some(square) = mask.least_significant_square() {
        mask = mask.unset_square(square);

        if pattern & (1 << i) != 0 {
            occupancy = occupancy.set_square(square);
        }

        i += 1;
    }

    occupancy
}

fn magic_number(square: Square, sliding_piece: SlidingPiece) -> u64 {
    let relevant_bits: u8 = match sliding_piece {
        SlidingPiece::Bishop => RELEVANT_BISHOP_OCCUPANCY_COUNT[square as usize],
        SlidingPiece::Rook => RELEVANT_ROOK_OCCUPANCY_COUNT[square as usize],
        _ => 0,
    };

    let mut occupancies = [BitBoard::EMPTY; 4096];
    let mut attacks = [BitBoard::EMPTY; 4096];
    let mut used_attacks: [BitBoard; 4096];

    let relevant_occupancy = match sliding_piece {
        SlidingPiece::Bishop => RELEVANT_BISHOP_OCCUPANCY[square as usize],
        SlidingPiece::Rook => RELEVANT_ROOK_OCCUPANCY[square as usize],
        _ => BitBoard::EMPTY,
    };

    let occupancy_index = 1 << relevant_bits;

    let mut index = 0;
    while index < occupancy_index {
        occupancies[index] = mask_occupancy(index as u64, relevant_occupancy);

        attacks[index] = match sliding_piece {
            SlidingPiece::Bishop => mask_bishop_attacks(square, occupancies[index]),
            SlidingPiece::Rook => mask_rook_attacks(square, occupancies[index]),
            _ => BitBoard::EMPTY,
        };

        index += 1;
    }

    loop {
        let magic_number = random_magic_number();

        if (relevant_occupancy.0.wrapping_mul(magic_number) & 0xFF00000000000000).count_ones() < 6 {
            continue;
        }

        used_attacks = [BitBoard::EMPTY; 4096];

        let mut index = 0;
        let mut fail = false;

        while index < occupancy_index && !fail {
            #[deny(long_running_const_eval)]
            let magic_index =
                (occupancies[index].0.wrapping_mul(magic_number)) >> (64 - relevant_bits);

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
