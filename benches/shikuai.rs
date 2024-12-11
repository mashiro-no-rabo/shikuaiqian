use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

// use shikuaiqian::*;

fn alloc(c: &mut Criterion) {
  c.bench_function("alloc", |b| {
    b.iter(|| shikuaiqian::alloc(black_box("5 62914 65 972 0 805922 6521 1639064")))
  });
}

fn opt(c: &mut Criterion) {
  c.bench_function("opt", |b| {
    b.iter(|| shikuaiqian::opt(black_box("5 62914 65 972 0 805922 6521 1639064")))
  });
}

criterion_group!(benches, alloc, opt);
criterion_main!(benches);