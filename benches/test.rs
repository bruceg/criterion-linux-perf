use criterion::{criterion_group, criterion_main, measurement::Measurement, Criterion};
use criterion_linux_perf::PerfMeasurement;

fn timeit<T: 'static + Measurement>(_crit: &mut Criterion<T>) {
    _crit.bench_function("String::new()", |_bencher| _bencher.iter(|| String::new()));

    _crit.bench_function(r#"String::from("")"#, |_bencher| {
        _bencher.iter(|| String::from(""))
    });

    _crit.bench_function(r#""".to_string()"#, |_bencher| {
        _bencher.iter(|| "".to_string())
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(PerfMeasurement);
    targets = timeit
);
criterion_main!(benches);
