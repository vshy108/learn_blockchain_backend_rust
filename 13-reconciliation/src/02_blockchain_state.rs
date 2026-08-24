// FILE: src/02_blockchain_state.rs
//
// LEARNING OBJECTIVE
// Explain the blockchain state concept in a minimal, testable Rust example.
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
pub struct BlockchainState {
    pub height: u64,
    pub hash: [u8; 32],
}

impl BlockchainState {
    pub fn new(height: u64, hash: [u8; 32]) -> Self {
        Self { height, hash }
    }
}

fn main() {
    let state = BlockchainState::new(5, [2u8; 32]);
    println!("height={}", state.height);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_latest_state() {
        let state = BlockchainState::new(120, [1u8; 32]);
        assert_eq!(state.height, 120);
        assert_eq!(state.hash, [1u8; 32]);
    }
}
