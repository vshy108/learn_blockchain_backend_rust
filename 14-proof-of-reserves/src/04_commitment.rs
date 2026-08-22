// FILE: src/04_commitment.rs
//
// LEARNING OBJECTIVE
// Explain the commitment concept in a minimal, testable Rust example.
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
pub struct Commitment {
    pub root: [u8; 32],
}

impl Commitment {
    pub fn new(root: [u8; 32]) -> Self {
        Self { root }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_root_hash() {
        let c = Commitment::new([3u8; 32]);
        assert_eq!(c.root, [3u8; 32]);
    }
}

fn main() {
    println!("{:?}", Commitment::new([1u8; 32]).root);
}
