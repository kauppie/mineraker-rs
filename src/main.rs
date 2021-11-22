mod area;
mod board;
mod position;
mod tile;

use crate::{
    board::{Board, BoardGenSeeder, BoardSeed, GenerationConfig},
    position::Position,
};

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
///
/// * Create [`Board`]s using builder pattern as [`Board`] is current version of it
/// is built once and then played.
/// ?? How would play-time generated board fit into this?
///
/// Move logic components into their own crates.

fn main() {
    let mut board = Board::new(&GenerationConfig {
        seed: BoardSeed::from_u128(0),
        width: 30,
        height: 16,
        mine_count: 99,
        start_pos: Position::default(),
    });

    board.open_from(Position { x: 15, y: 5 });

    println!("{}", board);
}
