extern crate criterion;
extern crate xdelta;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = "src.txt";
    let delta = "delta.txt";
    let output = "output_1.txt";
    
    println!("Patching delta target file: {}, from file {} using the file {}", &output, &input, &delta);
    c.bench_function("decode 1", |b| b.iter(|| xdelta::decode_file(Some(&input), &delta, &output)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);