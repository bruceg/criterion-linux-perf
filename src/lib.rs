//! This is a measurement plugin for
//! [Criterion.rs](https://bheisler.github.io/criterion.rs/book/index.html)
//! that provides measurements using Linux's perf interface.
//!
//! # Example
//!
//! ```
//! use criterion::{criterion_group, criterion_main, Criterion};
//! use criterion_linux_perf::{PerfMeasurement, PerfMode};
//!
//! fn timeit(crit: &mut Criterion<PerfMeasurement>) {
//!     crit.bench_function("String::new", |b| b.iter(|| String::new()));
//!     crit.bench_function("String::from", |b| b.iter(|| String::from("")));
//! }
//!
//! criterion_group!(
//!     name = benches;
//!     config = Criterion::default().with_measurement(
//!         PerfMeasurement::new(PerfMode::Branches),
//!     );
//!     targets = timeit
//! );
//! criterion_main!(benches);
//! ```

#![deny(missing_docs)]

use criterion::{
    measurement::{Measurement, ValueFormatter},
    Throughput,
};
use perf_event::{
    events::{Event, Hardware},
    Counter,
};

macro_rules! perf_mode {
    ( $( $ident:ident = $event:expr => $unit:literal, )* ) => {
        impl PerfMode {
            fn event(&self) -> Event {
                match self {
                    $( Self::$ident => $event.into(), )*
                }
            }

             fn formatter(&self) -> PerfFormatter {
                match self {
                    $( Self::$ident => (
                        PerfFormatter {
                            units: $unit,
                            throughput_bytes: concat!($unit, "/byte"),
                            throughput_elements: concat!($unit, "/element"),
                        }
                    ), )*
                }
            }
        }
    };
}

/// The perf counter to measure when running a benchmark.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PerfMode {
    /// The number of instructions retired. These can be affected by
    /// various issues, most notably hardware interrupt counts.
    Instructions,
    /// The total number of CPU cycles. This can be affected by CPU
    /// frequency scaling.
    Cycles,
    /// The number of branch instructions retired.
    Branches,
    /// The number of mispredicted branches.
    BranchMisses,
    /// The number of cache accesses.
    CacheRefs,
    /// The number of cache misses.
    CacheMisses,
    /// The number of bus cycles elapsed.
    BusCycles,
    /// The total number of CPU cycles elapsed. This is not affected by
    /// CPU frequency scaling.
    RefCycles,
}

perf_mode! {
    Instructions = Hardware::INSTRUCTIONS => "instructions",
    Cycles = Hardware::CPU_CYCLES => "cycles",
    Branches = Hardware::BRANCH_INSTRUCTIONS => "branches",
    BranchMisses = Hardware::BRANCH_MISSES => "branch misses",
    CacheRefs = Hardware::CACHE_REFERENCES => "cache refs",
    CacheMisses = Hardware::CACHE_MISSES => "cache misses",
    BusCycles = Hardware::BUS_CYCLES => "bus cycles",
    RefCycles = Hardware::REF_CPU_CYCLES => "cycles",
}

/// The measurement type to be used with `Criterion::with_measurement()`.
///
/// The default measurement created by `PerfMeasurement::default()` is
/// [`PerfMode`]`::Instructions`.
#[derive(Clone)]
pub struct PerfMeasurement {
    event: Event,
    formatter: PerfFormatter,
}

impl Default for PerfMeasurement {
    fn default() -> Self {
        Self::new(PerfMode::Instructions)
    }
}

impl PerfMeasurement {
    /// Create a new measurement, using the given [`PerfMode`] event.
    pub fn new(mode: PerfMode) -> Self {
        let event = mode.event();
        let formatter = mode.formatter();
        Self { event, formatter }
    }
}

impl Measurement for PerfMeasurement {
    type Intermediate = Counter;
    type Value = u64;

    fn start(&self) -> Self::Intermediate {
        let mut counter = perf_event::Builder::new()
            .kind(self.event.clone())
            .build()
            .unwrap();
        counter.enable().unwrap();
        counter
    }

    fn end(&self, mut counter: Self::Intermediate) -> Self::Value {
        counter.disable().unwrap();
        counter.read().unwrap()
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        v1 + v2
    }

    fn zero(&self) -> Self::Value {
        0
    }

    fn to_f64(&self, val: &Self::Value) -> f64 {
        *val as f64
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &self.formatter
    }
}

#[derive(Clone)]
struct PerfFormatter {
    units: &'static str,
    throughput_bytes: &'static str,
    throughput_elements: &'static str,
}

impl ValueFormatter for PerfFormatter {
    fn scale_values(&self, _typical_value: f64, _values: &mut [f64]) -> &'static str {
        self.units
    }

    fn scale_throughputs(
        &self,
        _typical_value: f64,
        throughput: &Throughput,
        values: &mut [f64],
    ) -> &'static str {
        match throughput {
            Throughput::Bytes(n) => {
                for val in values {
                    *val /= *n as f64;
                }
                self.throughput_bytes
            }
            Throughput::BytesDecimal(n) => {
                for val in values {
                    *val /= *n as f64;
                }
                self.throughput_bytes
            }
            Throughput::Elements(n) => {
                for val in values {
                    *val /= *n as f64;
                }
                self.throughput_elements
            }
        }
    }

    fn scale_for_machines(&self, _values: &mut [f64]) -> &'static str {
        self.units
    }
}
