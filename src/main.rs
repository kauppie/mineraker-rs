mod board;
mod tile;

use std::collections::HashSet;

use crate::board::{Board, BoardGenSeeder, BoardSeed, GenerationConfig, Position};

/// * Fix board generation to be unique for all seeds.
///   LSB of the seed doesn't currently have any effect on the generation.
/// * Use width and height when seeding board generation.
/// * Determine the requirements for board solving.
///
/// * Opening and flagging tiles.
/// * Cascade opening tiles and determining opened/unopened edges.
/// * Locally undeducable mine positions are stored and can be compared by
///   other tiles.
/// * Efficient algorithms using mainly iterators.

fn main() {
    /*
    let mut board = Board::new(&GenerationConfig {
        seed: BoardSeed::from_u128(0),
        width: 30,
        height: 16,
        mine_count: 99,
        start_pos: Position::default(),
    });

    board.open_from(Position { x: 15, y: 5 });

    println!("{}", board);
    */

    let mut set = HashSet::new();
    for i in 0..10 {
        set.insert(i);
    }
    let mut set2 = HashSet::new();
    for i in 4..15 {
        set2.insert(i);
    }

    println!("{:?}", set);
    println!("{:?}", set2);
    println!("{:?}", set.intersection(&set2));
}
