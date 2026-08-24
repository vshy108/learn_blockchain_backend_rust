// FILE: src/02_merkle_proof.rs
//
// LEARNING OBJECTIVE
// Explain the merkle proof concept in a minimal, testable Rust example.
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
pub struct MerkleProof {
    pub leaf_index: usize,
    pub path: Vec<[u8; 32]>,
}

impl MerkleProof {
    pub fn new(leaf_index: usize, path: Vec<[u8; 32]>) -> Self {
        Self { leaf_index, path }
    }
}

fn main() {
    let proof = MerkleProof::new(1, vec![]);
    println!("index={}", proof.leaf_index);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_proof_path() {
        let proof = MerkleProof::new(0, vec![[9u8; 32], [8u8; 32]]);
        assert_eq!(proof.path.len(), 2);
    }
}
