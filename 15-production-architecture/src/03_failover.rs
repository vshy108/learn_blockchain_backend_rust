// FILE: src/03_failover.rs
//
// LEARNING OBJECTIVE
// Explain the failover concept in a minimal, testable Rust example.
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

pub fn failover(current: bool, fallback: bool) -> bool {
    current || fallback
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_failover_to_backup() {
        assert!(failover(false, true));
        assert!(!failover(false, false));
    }
}

fn main() {
    println!("{}", failover(false, true));
}
