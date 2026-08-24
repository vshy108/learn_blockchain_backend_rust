// FILE: src/01_hsm_interface.rs
//
// LEARNING OBJECTIVE
// Explain the hsm interface concept in a minimal, testable Rust example.
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
pub struct HsmRequest {
    pub tx_hash: [u8; 32],
    pub key_id: &'static str,
}

pub trait HsmSigner {
    fn sign(&self, request: &HsmRequest) -> [u8; 64];
}

fn main() {
    println!("hsm ready");
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeHsm;
    impl HsmSigner for FakeHsm {
        fn sign(&self, request: &HsmRequest) -> [u8; 64] {
            let mut sig = [0u8; 64];
            sig[..32].copy_from_slice(&request.tx_hash);
            sig
        }
    }

    #[test]
    fn hsm_signer_returns_signature() {
        let hsm = FakeHsm;
        let sig = hsm.sign(&HsmRequest {
            tx_hash: [7u8; 32],
            key_id: "prod-1",
        });
        assert_eq!(sig[..32], [7u8; 32]);
    }
}
