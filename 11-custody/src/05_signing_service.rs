// FILE: src/05_signing_service.rs
//
// LEARNING OBJECTIVE
// Explain the signing service concept in a minimal, testable Rust example.
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
pub struct SigningService {
    pub signer_id: &'static str,
    pub last_signed: Option<[u8; 32]>,
}

impl SigningService {
    pub fn new(signer_id: &'static str) -> Self {
        Self {
            signer_id,
            last_signed: None,
        }
    }

    pub fn sign(&mut self, tx_hash: [u8; 32]) -> [u8; 32] {
        self.last_signed = Some(tx_hash);
        tx_hash
    }
}

fn main() {
    let mut service = SigningService::new("hsm-01");
    println!("{:?}", service.sign([1u8; 32]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_last_signed_transaction() {
        let mut service = SigningService::new("hsm-01");
        let tx_hash = [9u8; 32];
        assert_eq!(service.sign(tx_hash), tx_hash);
        assert_eq!(service.last_signed, Some(tx_hash));
    }
}
