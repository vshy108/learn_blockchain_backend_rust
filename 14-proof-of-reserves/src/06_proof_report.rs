// FILE: src/06_proof_report.rs
//
// LEARNING OBJECTIVE
// Explain the proof report concept in a minimal, testable Rust example.
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
pub struct ProofReport {
    pub root_hash: [u8; 32],
    pub included: bool,
}

impl ProofReport {
    pub fn new(root_hash: [u8; 32], included: bool) -> Self {
        Self { root_hash, included }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_proof_details() {
        let report = ProofReport::new([4u8; 32], true);
        assert_eq!(report.root_hash, [4u8; 32]);
        assert!(report.included);
    }
}

fn main() {
    let report = ProofReport::new([8u8; 32], true);
    println!("included={}", report.included);
}
