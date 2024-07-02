//! Forsyth-Edwards Notation (FEN) parser and formatter
//!
//! [FEN](https://www.chessprogramming.org/Forsyth-Edwards_Notation) is a
//! standard notation for describing a particular board position of a chess
//! game. It is used to describe the initial position of a game, as well as any
//! position during the game.
//!
//! # Errors
//!
//! Returns a [`FenError`] if the FEN string is invalid.
//!
//! # Examples
//!
//! ```
//! # use chess_engine::BoardBuilder;
//! let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
//! let board: BoardBuilder = fen_str.parse().unwrap();
//!
//! assert_eq!(board.to_string(), fen_str);
//! ```

use super::{BoardBuilder, CastleRightsError};
use crate::{
    Color, File, Piece, PieceError, PieceType, Player, PlayerError, Rank, Square, SquareError,
    MAX_HALFMOVE_CLOCK, MAX_PAWNS_PER_PLAYER, MAX_PIECES_PER_PLAYER, SQUARES,
};
use std::{fmt, str::FromStr};
use thiserror::Error;

/// Errors that can occur when parsing a FEN string
#[derive(Error, Debug)]
pub enum FenError {
    /// Invalid number of FEN sections
    #[error("invalid number of FEN sections (expected 4-6, got {0})")]
    Sections(usize),

    /// Invalid number of ranks
    #[error("invalid number of ranks (expected 8, got {0})")]
    Ranks(usize),

    /// Invalid number of files
    #[error("invalid number of files (expected 8, got {0})")]
    Files(usize),

    /// Invalid piece
    #[error("{0}")]
    Piece(#[from] PieceError),

    /// Pawns cannot be on the first or last rank
    #[error("pawns cannot be on the first or last rank")]
    PawnOnFirstOrLastRank,

    /// Player has too many pawns
    #[error("player {player} has too many pawns (expected 8 or fewer, got {num_pawns})")]
    ToManyPawns {
        /// [`Player`] with too many pawns
        player: Player,
        /// Number of pawns
        num_pawns: u8,
    },

    /// Player has too many pieces
    #[error("player {player} has too many pieces (expected 16 or fewer, got {num_pieces})")]
    ToManyPieces {
        /// [`Player`] with too many pieces
        player: Player,
        /// Number of pieces
        num_pieces: u8,
    },

    /// Invalid player
    #[error("{0}")]
    Player(#[from] PlayerError),

    /// Invalid castle rights
    #[error("{0}")]
    CastleRights(#[from] CastleRightsError),

    /// Invalid square
    #[error("{0}")]
    EnPassantSquare(#[from] SquareError),

    /// Invalid en passant rank
    #[error("invalid en passant rank (expected 3 or 6, got {0})")]
    EnPassantRank(Rank),

    /// Invalid halfmove clock
    #[error("invalid halfmove clock")]
    HalfmoveClock,

    /// Invalid fullmove counter
    #[error("invalid fullmove counter")]
    FullmoveCounter,
}

/// Parses a [`BoardBuilder`] from a [`FEN`] string
///
/// # Errors
///
/// Returns a [`FenError`] if the FEN string is invalid.
///
/// # Examples
///
/// ```
/// # use chess_engine::BoardBuilder;
/// let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
/// let board: BoardBuilder = fen_str.parse().unwrap();
/// assert_eq!(board.to_string(), fen_str);
/// ```
///
/// [`FEN`]: fen/index.html
impl FromStr for BoardBuilder {
    type Err = FenError;

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

                if rank != Rank::Three && rank != Rank::Six {
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

            if halfmove_clock > MAX_HALFMOVE_CLOCK {
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
///
/// Returns a vector of strings, where each string is a section of the FEN
/// string.
fn split_fen_string(fen: &str) -> Result<Vec<&str>, FenError> {
    let fen: Vec<&str> = fen.split_whitespace().collect();

    if fen.len() < 4 || fen.len() > 6 {
        return Err(FenError::Sections(fen.len()));
    }

    Ok(fen)
}

/// Parses the piece placement section of a FEN string
///
/// Returns an array of pieces, where the index is the square index on the
/// board and the value is the piece on that square.
fn piece_placement(piece_section: &str) -> Result<[Option<Piece>; SQUARES], FenError> {
    let mut pieces = [None; SQUARES];

    let ranks: Vec<&str> = piece_section.split('/').collect();

    if ranks.len() != Rank::LEN {
        return Err(FenError::Ranks(ranks.len()));
    }

    let (mut num_white_pawns, mut num_white_pieces) = (0, 0);
    let (mut num_black_pawns, mut num_black_pieces) = (0, 0);

    for (rank_index, rank) in ranks.iter().rev().enumerate() {
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

            pieces[Square::new(File::new(file_index), Rank::new(rank_index)).0 as usize] =
                Some(piece);
            file_index += 1;
        }

        if file_index != File::LEN {
            return Err(FenError::Files(file_index));
        }
    }

    if num_white_pieces > MAX_PIECES_PER_PLAYER {
        return Err(FenError::ToManyPieces {
            player: Player(Color::White),
            num_pieces: num_white_pieces as u8,
        });
    }

    if num_black_pieces > MAX_PIECES_PER_PLAYER {
        return Err(FenError::ToManyPieces {
            player: Player(Color::Black),
            num_pieces: num_black_pieces as u8,
        });
    }

    if num_white_pawns > MAX_PAWNS_PER_PLAYER {
        return Err(FenError::ToManyPawns {
            player: Player(Color::White),
            num_pawns: num_white_pawns as u8,
        });
    }

    if num_black_pawns > MAX_PAWNS_PER_PLAYER {
        return Err(FenError::ToManyPawns {
            player: Player(Color::Black),
            num_pawns: num_black_pawns as u8,
        });
    }

    Ok(pieces)
}

/// Formats a [`BoardBuilder`] as a [`FEN`] string
///
/// # Examples
/// ```
/// # use chess_engine::BoardBuilder;
/// let board = BoardBuilder::new();
/// assert_eq!(
///    board.to_string(),
///   "8/8/8/8/8/8/8/8 w - - 0 1"
/// );
/// ```
///
/// [`FEN`]: fen/index.html
impl fmt::Display for BoardBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        for rank in (0..Rank::LEN).rev() {
            let mut empty = 0;

            for file in 0..File::LEN {
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
            self.state.player(),
            self.state.castling_rights(),
            self.state
                .en_passant_square()
                .map_or_else(|| "-".to_string(), |s| s.to_string()),
            self.state.halfmove_clock(),
            self.state.fullmove_counter()
        )
    }
}
