#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

//! Types for working with chessboards.

mod bitboard;
mod castle_rights_type;
mod color;
mod file;
pub(crate) mod macros;
mod piece;
mod piece_type;
mod rand;
mod rank;
mod sliding_piece;
mod square;

pub use bitboard::*;
pub use castle_rights_type::*;
pub use color::*;
pub use file::*;
pub use piece::*;
pub use piece_type::*;
pub use rand::*;
pub use rank::*;
pub use sliding_piece::*;
pub use square::*;
