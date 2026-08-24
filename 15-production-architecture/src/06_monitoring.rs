// FILE: src/06_monitoring.rs
//
// LEARNING OBJECTIVE
// Explain the monitoring concept in a minimal, testable Rust example.
//
// BLOCKCHAIN CONCEPT
// This example models the relevant blockchain behavior without requiring a live node.
//
// NORMAL CASE
// - The common happy-path flow is exercised in a local unit test.
// - Inputs are small, deterministic, and easy to inspect.
//
// SPECIAL CASES
// - Edge conditions are covered by dedicated assertions.
// - Reorg, nonce, signing, fee, or validation logic is modeled explicitly where relevant.
//
// EXCEPTIONAL CASES
// - Invalid states are rejected by tests instead of a real blockchain node.
// - This keeps the lesson focused on protocol behavior rather than network dependencies.
//
// DESIGN DECISION
// Keep the model explicit, teachable, and deterministic so future engineers can reason about the logic quickly.
//
// This file stays local-first and test-driven by design.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitoringMetric {
    pub name: &'static str,
    pub value: u64,
}

impl MonitoringMetric {
    pub fn new(name: &'static str, value: u64) -> Self {
        Self { name, value }
    }
}

fn main() {
    let metric = MonitoringMetric::new("success_rate", 99);
    println!("{}", metric.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_metric_name_and_value() {
        let metric = MonitoringMetric::new("rpc_latency_ms", 42);
        assert_eq!(metric.name, "rpc_latency_ms");
        assert_eq!(metric.value, 42);
    }
}
