use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mineraker::Board;

pub fn bench_generate_board_shuffling(c: &mut Criterion) {
    c.bench_function("Shuffling board generation", |b| {
        b.iter(|| black_box(Board::new(30, 16, 170, 0)))
    });
}

pub fn bench_generate_board_choosing(c: &mut Criterion) {
    c.bench_function("Index choosing board generation", |b| {
        b.iter(|| black_box(Board::new_choosing(30, 16, 170, 0)))
    });
}

criterion_group!(
    benches,
    bench_generate_board_shuffling,
    bench_generate_board_choosing
);
criterion_main!(benches);
