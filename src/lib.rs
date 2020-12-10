use criterion::{
    measurement::{Measurement, ValueFormatter},
    Throughput,
};
use perf_event::Counter;

pub struct PerfMeasurement;

impl Measurement for PerfMeasurement {
    type Intermediate = Counter;
    type Value = u64;

    fn start(&self) -> Self::Intermediate {
        let mut counter = perf_event::Builder::new().build().unwrap();
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
        &PerfFormatter
    }
}

pub struct PerfFormatter;

impl ValueFormatter for PerfFormatter {
    fn format_value(&self, value: f64) -> String {
        format!("{:.0} insns", value)
    }

    fn format_throughput(&self, throughput: &Throughput, value: f64) -> String {
        match throughput {
            Throughput::Bytes(b) => format!("{:.4} insns/byte", value / *b as f64),
            Throughput::Elements(e) => format!("{:.0} insns/{}", value, e),
        }
    }

    fn scale_values(&self, _typical_value: f64, _values: &mut [f64]) -> &'static str {
        "insns"
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
                "insns/byte"
            }
            Throughput::Elements(n) => {
                for val in values {
                    *val /= *n as f64;
                }
                "insns/element"
            }
        }
    }

    fn scale_for_machines(&self, _values: &mut [f64]) -> &'static str {
        "insns"
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
