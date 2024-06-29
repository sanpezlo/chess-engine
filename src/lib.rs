#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! A chess engine library
//!
//! This library is a personal project to learn
//! [Chess Programming](https://www.chessprogramming.org). It is a work in
//! progress and is not intended to be used in production. For real world
//! applications, consider using [Stockfish](https://stockfishchess.org/) or
//! [Leela Chess Zero](https://lczero.org/).
//!
//! For the development of this library, I am following the book
//! [Creating the Rustic chess engine](rustic-chess.org) and the
//! [Chess Programming Wiki](https://www.chessprogramming.org), as well as
//! repositories like [Chess](https://github.com/jordanbray/chess),
//! [Pleco](https://github.com/pleco-rs/Pleco) and
//! [Cozy-chess](https://github.com/analog-hors/cozy-chess).

pub mod board;
pub mod core;

pub use crate::board::*;
pub use crate::core::*;
