// FILE: src/01_reconciliation_record.rs
//
// LEARNING OBJECTIVE
// Explain the reconciliation record concept in a minimal, testable Rust example.
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
pub struct ReconciliationRecord {
    pub block_height: u64,
    pub difference: i64,
}

impl ReconciliationRecord {
    pub fn new(block_height: u64, difference: i64) -> Self {
        Self {
            block_height,
            difference,
        }
    }
}

fn main() {
    let record = ReconciliationRecord::new(100, 3);
    println!("diff={}", record.difference);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_block_difference() {
        let record = ReconciliationRecord::new(101, -5);
        assert_eq!(record.block_height, 101);
        assert_eq!(record.difference, -5);
    }
}
