// FILE: src/04_discrepancy.rs
//
// LEARNING OBJECTIVE
// Explain the discrepancy concept in a minimal, testable Rust example.
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

pub fn detect_discrepancy(on_chain: u64, in_ledger: u64) -> i64 {
    on_chain as i64 - in_ledger as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_positive_difference() {
        assert_eq!(detect_discrepancy(100, 90), 10);
    }
}

fn main() {
    println!("{}", detect_discrepancy(40, 35));
}
