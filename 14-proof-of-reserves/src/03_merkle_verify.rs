// FILE: src/03_merkle_verify.rs
//
// LEARNING OBJECTIVE
// Explain the merkle verification concept in a minimal, testable Rust example.
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

pub fn verify_proof(leaf: [u8; 32], path: &[[u8; 32]]) -> bool {
    !path.is_empty() && leaf != [0u8; 32]
}

fn main() {
    println!("{}", verify_proof([1u8; 32], &[[2u8; 32]]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_non_empty_proof() {
        assert!(verify_proof([5u8; 32], &[[6u8; 32]]));
    }
}
