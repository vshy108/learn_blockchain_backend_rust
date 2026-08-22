// FILE: src/02_withdrawal_request.rs
//
// LEARNING OBJECTIVE
// Explain the withdrawal request concept in a minimal, testable Rust example.
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
pub struct WithdrawalRequest {
    pub user_id: u64,
    pub amount: u64,
    pub to: [u8; 20],
}

impl WithdrawalRequest {
    pub fn new(user_id: u64, amount: u64, to: [u8; 20]) -> Self {
        Self { user_id, amount, to }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_withdrawal_request() {
        let req = WithdrawalRequest::new(7, 500, [1u8; 20]);
        assert_eq!(req.user_id, 7);
        assert_eq!(req.amount, 500);
    }
}

fn main() {
    let req = WithdrawalRequest::new(3, 10, [2u8; 20]);
    println!("{}", req.amount);
}
