use chess_engine::{BitBoard, BoardBuilder, Square};

fn main() {
    let square = Square(55);

    println!("Square: {}", square);
    println!("{}", BitBoard(18445336716005867520));

    let board_builder: BoardBuilder =
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2"
            .parse()
            .map_err(|e| {
                eprintln!("{}", e);
            })
            .unwrap();

    println!("{:?}", board_builder.build());
}
