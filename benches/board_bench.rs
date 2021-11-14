use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mineraker::{
    board::{Board, BoardGenSeeder, BoardSeed, GenerationConfig},
    position::Position,
};

pub fn bench_generate_small_board(c: &mut Criterion) {
    c.bench_function("Generate small board", |b| {
        b.iter(|| {
            black_box(Board::new(&GenerationConfig {
                seed: BoardSeed::from_u128(0),
                width: 8,
                height: 8,
                mine_count: 10,
                start_pos: Position::default(),
            }))
        })
    });
}

pub fn bench_generate_large_board(c: &mut Criterion) {
    c.bench_function("Generate large board", |b| {
        b.iter(|| {
            black_box(Board::new(&GenerationConfig {
                seed: BoardSeed::from_u128(0),
                width: 30,
                height: 16,
                mine_count: 170,
                start_pos: Position::default(),
            }))
        })
    });
}

pub fn bench_cascade_open(c: &mut Criterion) {
    c.bench_function("Cascade open", |b| {
        b.iter(|| {
            let mut board = Board::new(&GenerationConfig {
                seed: BoardSeed::from_u128(0),
                width: 30,
                height: 16,
                mine_count: 99,
                start_pos: Position::default(),
            });
            board.open_from(Position { x: 15, y: 5 });

            black_box(board)
        })
    });
}

criterion_group!(
    benches,
    bench_generate_small_board,
    bench_generate_large_board,
    bench_cascade_open,
);
criterion_main!(benches);
