use std::hint::black_box;
use backyard_parser::parse;
use criterion::{ criterion_group, criterion_main, Criterion };

const CONTENT: &str = "<?php
class A {
}";

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("sample", |b| {
    b.iter(|| {
      let _ = parse(black_box(&CONTENT));
    });
  });
}

criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(500);
  targets = criterion_benchmark,
}
criterion_main!(benches);
