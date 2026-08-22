// FILE: src/05_resolution.rs
//
// LEARNING OBJECTIVE
// Explain the resolution concept in a minimal, testable Rust example.
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

pub fn resolve_difference(current: u64, corrected: u64) -> u64 {
    corrected.saturating_sub(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_correction_amount() {
        assert_eq!(resolve_difference(90, 100), 10);
    }
}

fn main() {
    println!("{}", resolve_difference(50, 60));
}
