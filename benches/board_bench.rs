use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mineraker::board::{Board, BoardGenSeeder, BoardSeed, GenerationConfig, Position};

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

criterion_group!(
    benches,
    bench_generate_small_board,
    bench_generate_large_board,
);
criterion_main!(benches);
