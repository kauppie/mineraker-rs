use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mineraker::Board;

pub fn bench_generate_board(c: &mut Criterion) {
    c.bench_function("Generate large board", |b| {
        b.iter(|| black_box(Board::new(30, 16, 170, 0)))
    });
}

criterion_group!(benches, bench_generate_board);
criterion_main!(benches);
