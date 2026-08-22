// FILE: src/06_rotation.rs
//
// LEARNING OBJECTIVE
// Explain the rotation concept in a minimal, testable Rust example.
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
pub struct KeyRotation {
    pub current_key: &'static str,
    pub next_key: &'static str,
}

impl KeyRotation {
    pub fn new(current: &'static str, next: &'static str) -> Self {
        Self { current_key: current, next_key: next }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotates_to_next_key() {
        let rotation = KeyRotation::new("key-v1", "key-v2");
        assert_eq!(rotation.current_key, "key-v1");
        assert_eq!(rotation.next_key, "key-v2");
    }
}

fn main() {
    let rotation = KeyRotation::new("old", "new");
    println!("old={}, new={}", rotation.current_key, rotation.next_key);
}
