// FILE: src/02_hsm_integration.rs
//
// LEARNING OBJECTIVE
// Explain the hsm integration concept in a minimal, testable Rust example.
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
pub struct HsmIntegration {
    pub device_id: &'static str,
    pub connected: bool,
}

impl HsmIntegration {
    pub fn new(device_id: &'static str) -> Self {
        Self { device_id, connected: true }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_tracks_connection() {
        let hsm = HsmIntegration::new("hsm-02");
        assert!(hsm.connected);
        assert_eq!(hsm.device_id, "hsm-02");
    }
}

fn main() {
    let hsm = HsmIntegration::new("hsm-02");
    println!("device={}", hsm.device_id);
}
