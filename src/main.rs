mod board;

use board::Board;

fn main() {
    let board = Board::new(30, 16, 170, 0);

    println!("{}", board);
}
