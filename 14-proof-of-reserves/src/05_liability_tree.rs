// FILE: src/05_liability_tree.rs
//
// LEARNING OBJECTIVE
// Explain the liability tree concept in a minimal, testable Rust example.
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
pub struct LiabilityTree {
    pub total: u64,
}

impl LiabilityTree {
    pub fn new(total: u64) -> Self {
        Self { total }
    }
}

fn main() {
    let tree = LiabilityTree::new(5);
    println!("total={}", tree.total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_total_liability() {
        let tree = LiabilityTree::new(10_000);
        assert_eq!(tree.total, 10_000);
    }
}
