extern crate criterion;
extern crate xdelta;
extern crate pprof;
extern crate rand;

use criterion::{
    Criterion,
    Throughput,
    BenchmarkId,
    criterion_group,
    criterion_main
};

use pprof::criterion::{PProfProfiler, Output};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = "src.txt";
    let delta = "delta.txt";
    let output = "output_1.txt";
    
    println!("Patching delta target file: {}, from file {} using the file {}", &output, &input, &delta);
    c.bench_function("decode 1", |b| b.iter(|| xdelta::decode_file(Some(&input), &delta, &output)));
}

criterion_group!{
    name = benches;
    config = Criterion::default()
        .with_profiler(
            PProfProfiler::new(100, Output::Flamegraph(None))
        );
    targets = criterion_benchmark
}
criterion_main!(benches);