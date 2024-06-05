use super::{BoardBuilder, CastleRightsError};
use crate::{
    Color, File, Piece, PieceError, PieceType, Player, PlayerError, Rank, Square, SquareError,
};
use std::{fmt, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FenError {
    #[error("invalid number of FEN sections (expected 4-6, got {0})")]
    Sections(usize),

    #[error("invalid number of ranks (expected 8, got {0})")]
    Ranks(usize),

    #[error("invalid number of files (expected 8, got {0})")]
    Files(usize),

    #[error("{0}")]
    Piece(#[from] PieceError),

    #[error("pawns cannot be on the first or last rank")]
    PawnOnFirstOrLastRank,

    #[error("player {player} has too many pawns (expected 8 or fewer, got {num_pawns})")]
    ToManyPawns { player: Player, num_pawns: u8 },

    #[error("player {player} has too many pieces (expected 16 or fewer, got {num_pieces})")]
    ToManyPieces { player: Player, num_pieces: u8 },

    #[error("{0}")]
    Player(#[from] PlayerError),

    #[error("{0}")]
    CastleRights(#[from] CastleRightsError),

    #[error("{0}")]
    EnPassantSquare(#[from] SquareError),

    #[error("invalid en passant rank (expected 3 or 6, got {0})")]
    EnPassantRank(Rank),

    #[error("invalid halfmove clock")]
    HalfmoveClock,

    #[error("invalid fullmove counter")]
    FullmoveCounter,
}

impl FromStr for BoardBuilder {
    type Err = FenError;

    /// Constructs a board from a Forsyth-Edwards Notation (FEN)
    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        let mut board_builder = BoardBuilder::new();

        let fen = split_fen_string(fen)?;

        let pieces = piece_placement(fen[0])?;

        board_builder.pieces(pieces);

        board_builder.player(fen[1].parse()?);

        board_builder.castling_rights(fen[2].parse()?);

        let en_passant_square = {
            if fen[3] == "-" {
                None
            } else {
                let square: Square = fen[3].parse()?;
                let rank = square.rank();

                if rank as u8 != 2 && rank as u8 != 5 {
                    return Err(FenError::EnPassantRank(rank));
                }

                Some(square)
            }
        };

        board_builder.en_passant_square(en_passant_square);

        let mut halfmove_clock = 0;

        if fen.len() > 4 {
            halfmove_clock = fen[4].parse().map_err(|_| FenError::HalfmoveClock)?;

            // TODO: if is not a checkmate and halfmove_clock is 100, it is a draw

            if halfmove_clock > 100 {
                return Err(FenError::HalfmoveClock);
            }

            board_builder.halfmove_clock(halfmove_clock);
        } else {
            board_builder.halfmove_clock(0);
        }

        if fen.len() > 5 {
            let fullmove_counter: u16 = fen[5].parse().map_err(|_| FenError::FullmoveCounter)?;

            if fullmove_counter == 0 {
                return Err(FenError::FullmoveCounter);
            }

            if halfmove_clock as u16 > fullmove_counter * 2 {
                return Err(FenError::HalfmoveClock);
            }

            board_builder.fullmove_counter(fullmove_counter);
        } else {
            board_builder.fullmove_counter(1);
        }

        Ok(board_builder)
    }
}

/// Splits a FEN string into its sections
fn split_fen_string(fen: &str) -> Result<Vec<&str>, FenError> {
    let fen: Vec<&str> = fen.split_whitespace().collect();

    if fen.len() < 4 || fen.len() > 6 {
        return Err(FenError::Sections(fen.len()));
    }

    Ok(fen)
}

/// Parses the piece placement section of a FEN string
fn piece_placement(piece_section: &str) -> Result<[Option<Piece>; 64], FenError> {
    let mut pieces = [None; 64];

    let ranks: Vec<&str> = piece_section.split('/').collect();

    if ranks.len() != 8 {
        return Err(FenError::Ranks(ranks.len()));
    }

    let (mut num_white_pawns, mut num_white_pieces) = (0, 0);
    let (mut num_black_pawns, mut num_black_pieces) = (0, 0);

    for (rank_index, rank) in ranks.iter().enumerate() {
        let mut file_index = 0;

        for file in rank.chars() {
            if let Some(digit) = file.to_digit(10) {
                file_index += digit as usize;
                continue;
            }

            let piece: Piece = file.to_string().as_str().parse()?;

            if piece.player() == Player(Color::White) {
                num_white_pieces += 1;
            } else {
                num_black_pieces += 1;
            }

            if piece.piece_type() == PieceType::Pawn {
                if rank_index == 0 || rank_index == 7 {
                    return Err(FenError::PawnOnFirstOrLastRank);
                }

                if piece.player() == Player(Color::White) {
                    num_white_pawns += 1;
                } else {
                    num_black_pawns += 1;
                }
            }

            pieces[(7 - rank_index) * 8 + file_index] = Some(piece);
            file_index += 1;
        }

        if file_index != 8 {
            return Err(FenError::Files(file_index));
        }
    }

    if num_white_pieces > 16 {
        return Err(FenError::ToManyPieces {
            player: Player(Color::White),
            num_pieces: num_white_pieces,
        });
    }

    if num_black_pieces > 16 {
        return Err(FenError::ToManyPieces {
            player: Player(Color::Black),
            num_pieces: num_black_pieces,
        });
    }

    if num_white_pawns > 8 {
        return Err(FenError::ToManyPawns {
            player: Player(Color::White),
            num_pawns: num_white_pawns,
        });
    }

    if num_black_pawns > 8 {
        return Err(FenError::ToManyPawns {
            player: Player(Color::Black),
            num_pawns: num_black_pawns,
        });
    }

    Ok(pieces)
}

impl fmt::Display for BoardBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        for rank in (0..8).rev() {
            let mut empty = 0;

            for file in 0..8 {
                let square = Square::new(File::new(file), Rank::new(rank));

                if let Some(piece) = self.pieces[square.0 as usize] {
                    if empty > 0 {
                        s.push_str(&empty.to_string());
                        empty = 0;
                    }

                    s.push_str(&piece.to_string());
                } else {
                    empty += 1;
                }
            }

            if empty > 0 {
                s.push_str(&empty.to_string());
            }

            if rank > 0 {
                s.push('/');
            }
        }

        write!(
            f,
            "{} {} {} {} {} {}",
            s,
            self.player,
            self.castling_rights,
            self.en_passant_square
                .map_or_else(|| "-".to_string(), |s| s.to_string()),
            self.halfmove_clock,
            self.fullmove_counter
        )
    }
}
