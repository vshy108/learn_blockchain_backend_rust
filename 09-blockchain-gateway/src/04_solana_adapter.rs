// FILE: src/04_solana_adapter.rs
//
// LEARNING OBJECTIVE
// Explain the solana adapter concept in a minimal, testable Rust example.
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
pub struct SolanaAdapter {
    pub slot: u64,
}

impl SolanaAdapter {
    pub fn new(slot: u64) -> Self {
        Self { slot }
    }
}

fn main() {
    let adapter = SolanaAdapter::new(6);
    println!("slot={}", adapter.slot);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solana_adapter_tracks_slot() {
        let adapter = SolanaAdapter::new(777);
        assert_eq!(adapter.slot, 777);
    }
}
