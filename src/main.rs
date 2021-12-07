mod area;
mod board;
mod position;
mod tile;

use crate::{
    board::{Board, GenerationSettings, Seed},
    position::Position,
};

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
///
/// If Area::next_action produces `None`, then Area itself is unusable.
/// Area has to be used with other areas.
/// TODO: How to determine these areas?

fn main() {
    let mut board = Board::new(&GenerationSettings {
        seed: Seed::new(0),
        width: 30,
        height: 16,
        mine_count: 99,
        start_pos: Position::default(),
    });

    board.open_from(Position { x: 15, y: 5 });

    println!("{}", board);
}
