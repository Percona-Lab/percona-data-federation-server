use criterion::{Criterion, criterion_group, criterion_main};

fn bench_factorial(c: &mut Criterion) {
    c.bench_function("factorial", |b| {
        b.iter(|| percona_data_federation_server::factorial::compute(std::hint::black_box(100000)));
    });
}

criterion_group!(benches, bench_factorial);
criterion_main!(benches);
