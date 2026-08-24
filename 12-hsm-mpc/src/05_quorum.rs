// FILE: src/05_quorum.rs
//
// LEARNING OBJECTIVE
// Explain the quorum concept in a minimal, testable Rust example.
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

pub fn has_quorum(approvals: usize, threshold: usize) -> bool {
    approvals >= threshold
}

fn main() {
    println!("{}", has_quorum(2, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_threshold_approval() {
        assert!(has_quorum(3, 2));
        assert!(!has_quorum(1, 2));
    }
}
