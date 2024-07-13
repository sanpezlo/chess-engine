#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

//! Move generation library for the chess engine.

mod board;
mod board_builder;
mod castle_rights;
mod draw;
pub mod fen;
mod magic;
mod movegen;
mod state;
mod zobrist;

pub use board::*;
pub use board_builder::*;
pub use castle_rights::*;
pub use magic::*;
pub use state::*;
pub use zobrist::*;
