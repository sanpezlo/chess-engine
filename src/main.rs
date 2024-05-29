use std::str::FromStr;

use chess_engine::{BitBoard, Square};

fn main() {
    let square = Square::from_str("a2").unwrap_or_else(|s| {
        eprintln!("{}", s);
        std::process::exit(1);
    });

    println!("Square: {}", square);
    println!("{}", BitBoard(1u64 << square.0));
}
