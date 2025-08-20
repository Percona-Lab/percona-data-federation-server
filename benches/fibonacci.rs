use criterion::{Criterion, criterion_group, criterion_main};

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci", |b| {
        b.iter(|| percona_data_federation_server::fibonacci::compute(std::hint::black_box(100000)));
    });
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
