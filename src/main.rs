mod board;
mod tile;

use crate::board::{Board, BoardGenSeeder, BoardSeed, GenerationConfig, Position};

fn main() {
    let board = Board::new(&GenerationConfig {
        seed: BoardSeed::from_u128(0),
        width: 30,
        height: 16,
        mine_count: 170,
        start_pos: Position::default(),
    });

    println!("{}", board);
}
