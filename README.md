[![crates.io version](https://meritbadge.herokuapp.com/criterion-linux-perf)](https://crates.io/crates/criterion-linux-perf)
[![docs.rs](https://docs.rs/criterion-linux-perf/badge.svg)](https://docs.rs/criterion-linux-perf)

# criterion-linux-perf

This is a measurement plugin for [Criterion.rs](https://bheisler.github.io/criterion.rs/book/index.html) that provides measurements using Linux's perf interface.

## Supported Events

criterion-linux-perf uses the
[`perf-event`](https://github.com/jimblandy/perf-event) crate and
supports a subset of the events provided by that crate. If you require
more events than the current selection, please [open an
issue](https://github.com/bruceg/criterion-linux-perf/issues) to request
additions.

## Example

The following code shows on how to count branches when creating an empty string:

```rust
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
```

## Other Crates

I am aware of one other crate that provides the same functionality,
[`criterion-perf-events`](https://github.com/jbreitbart/criterion-perf-events). While
it provides a much wider coverage of the available perf event types, it
depends on [`perfcnt`](https://github.com/gz/rust-perfcnt) which only
builds on Rust nightly. This crate depends on
[`perf-event`](https://github.com/jimblandy/perf-event), which does not
have that limitation.
