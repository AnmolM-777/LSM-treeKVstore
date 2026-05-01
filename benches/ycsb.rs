use criterion::{criterion_group, criterion_main, Criterion};
fn bench_ycsb_a(c: &mut Criterion) { c.bench_function("YCSB-A", |b| b.iter(|| {})); }
criterion_group!(benches, bench_ycsb_a);
criterion_main!(benches);
