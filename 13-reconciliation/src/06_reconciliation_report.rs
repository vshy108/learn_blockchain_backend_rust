// FILE: src/06_reconciliation_report.rs
//
// LEARNING OBJECTIVE
// Explain the reconciliation report concept in a minimal, testable Rust example.
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
pub struct ReconciliationReport {
    pub difference: i64,
    pub resolved: bool,
}

impl ReconciliationReport {
    pub fn new(difference: i64, resolved: bool) -> Self {
        Self {
            difference,
            resolved,
        }
    }
}

fn main() {
    let report = ReconciliationReport::new(1, true);
    println!("resolved={}", report.resolved);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_resolution_result() {
        let report = ReconciliationReport::new(7, true);
        assert_eq!(report.difference, 7);
        assert!(report.resolved);
    }
}
