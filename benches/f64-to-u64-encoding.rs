use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ore_encoding_rs::OrePlaintext;

#[inline]
fn do_encode(input: f64) -> OrePlaintext<u64> {
  OrePlaintext::<u64>::from(input)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("OrePlaintext::<u64>::from(f64)", |b| {
        b.iter(|| do_encode(black_box(123.4567)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
