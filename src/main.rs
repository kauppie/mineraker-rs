mod board;

use board::Board;

fn main() {
    let board = Board::new(8, 8, 10, 0);

    println!("{}", board);
}
