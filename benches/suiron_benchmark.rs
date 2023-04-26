use criterion::{criterion_group, criterion_main, Criterion};
use suiron::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("benchmark", |b| b.iter(|| benchmark()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
