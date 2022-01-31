use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ore_encoding_rs::siphash;

#[inline]
fn do_hash(input: &[u8]) -> u64 {
  siphash(input)
}

fn criterion_benchmark(c: &mut Criterion) {
    let string = "The quick brown fox jumped over the lazy dogs".as_bytes();
    c.bench_function("siphash", |b| {
        b.iter(|| do_hash(black_box(string)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
