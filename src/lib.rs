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
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum PerfMode {
            $( $ident, )*
        }

        impl PerfMode {
            fn event(&self) -> Event {
                match self {
                    $( Self::$ident => $event.into(), )*
                }
            }

             fn units(&self) -> (&'static str, &'static str, &'static str) {
                match self {
                    $( Self::$ident => (
                        $unit,
                        concat!($unit, "/byte"),
                        concat!($unit, "/element"),
                    ), )*
                }
            }
        }
    };
}

perf_mode! {
    Instructions = Hardware::INSTRUCTIONS => "instructions",
    Cycles = Hardware::CPU_CYCLES => "cycles",
    Branches = Hardware::BRANCH_INSTRUCTIONS => "branches",
    BranchMisses = Hardware::BRANCH_MISSES => "branch misses",
    CacheRefs = Hardware::CACHE_REFERENCES => "cache refs",
    CacheMisses = Hardware::CACHE_MISSES => "cache misses",
    BusCycles = Hardware::BUS_CYCLES => "bus cycles",
}

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
    pub fn new(mode: PerfMode) -> Self {
        let units = mode.units();
        let event = mode.event();
        let formatter = PerfFormatter {
            units: units.0,
            throughput_bytes: units.1,
            throughput_elements: units.2,
        };
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
pub struct PerfFormatter {
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
