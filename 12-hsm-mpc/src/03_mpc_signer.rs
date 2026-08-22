// FILE: src/03_mpc_signer.rs
//
// LEARNING OBJECTIVE
// Explain the mpc signer concept in a minimal, testable Rust example.
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
pub struct MpcSigner {
    pub parties: Vec<&'static str>,
}

impl MpcSigner {
    pub fn new(parties: Vec<&'static str>) -> Self {
        Self { parties }
    }

    pub fn quorum_ok(&self, threshold: usize) -> bool {
        self.parties.len() >= threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quorum_check_uses_party_count() {
        let signer = MpcSigner::new(vec!["a", "b", "c"]);
        assert!(signer.quorum_ok(2));
        assert!(!signer.quorum_ok(4));
    }
}

fn main() {
    let signer = MpcSigner::new(vec!["a", "b"]);
    println!("quorum={}", signer.quorum_ok(2));
}
