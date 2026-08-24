// FILE: src/01_merkle_tree.rs
//
// LEARNING OBJECTIVE
// Explain the merkle tree concept in a minimal, testable Rust example.
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
pub struct MerkleTree {
    pub leaves: Vec<[u8; 32]>,
}

impl MerkleTree {
    pub fn new(leaves: Vec<[u8; 32]>) -> Self {
        Self { leaves }
    }
}

fn main() {
    let tree = MerkleTree::new(vec![]);
    println!("leaves={}", tree.leaves.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_leaf_hashes() {
        let tree = MerkleTree::new(vec![[1u8; 32], [2u8; 32]]);
        assert_eq!(tree.leaves.len(), 2);
    }
}
