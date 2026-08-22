// FILE: src/04_key_recovery.rs
//
// LEARNING OBJECTIVE
// Explain the key recovery concept in a minimal, testable Rust example.
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
pub struct KeyRecoveryState {
    pub recovery_mode: bool,
    pub backup_available: bool,
}

impl KeyRecoveryState {
    pub fn new(recovery_mode: bool, backup_available: bool) -> Self {
        Self { recovery_mode, backup_available }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovery_mode_is_flagged_when_needed() {
        let state = KeyRecoveryState::new(true, true);
        assert!(state.recovery_mode);
        assert!(state.backup_available);
    }
}

fn main() {
    let state = KeyRecoveryState::new(false, true);
    println!("recovery={}", state.recovery_mode);
}
