use criterion::{criterion_group, criterion_main, Criterion};
use criterion_linux_perf::{PerfMeasurement, PerfMode};

fn timeit(crit: &mut Criterion<PerfMeasurement>) {
    crit.bench_function("String::new", |b| b.iter(|| String::new()));
    crit.bench_function("String::from", |b| b.iter(|| String::from("")));
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(PerfMeasurement::new(PerfMode::Branches));
    targets = timeit
);
criterion_main!(benches);
