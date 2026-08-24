// FILE: src/04_nonce_manager.rs
//
// LEARNING OBJECTIVE
// Explain the nonce manager concept in a minimal, testable Rust example.
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
pub struct NonceManager {
    pub next_nonce: u64,
}

impl NonceManager {
    pub fn new(start: u64) -> Self {
        Self { next_nonce: start }
    }

    pub fn reserve(&mut self) -> u64 {
        let nonce = self.next_nonce;
        self.next_nonce += 1;
        nonce
    }
}

fn main() {
    let mut manager = NonceManager::new(1);
    println!("{}", manager.reserve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issues_unique_nonces() {
        let mut manager = NonceManager::new(7);
        assert_eq!(manager.reserve(), 7);
        assert_eq!(manager.reserve(), 8);
    }
}
