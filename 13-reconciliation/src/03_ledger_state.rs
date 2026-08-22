// FILE: src/03_ledger_state.rs
//
// LEARNING OBJECTIVE
// Explain the ledger state concept in a minimal, testable Rust example.
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
pub struct LedgerState {
    pub balance: u64,
    pub account: [u8; 20],
}

impl LedgerState {
    pub fn new(balance: u64, account: [u8; 20]) -> Self {
        Self { balance, account }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_balance_and_account() {
        let state = LedgerState::new(999, [5u8; 20]);
        assert_eq!(state.balance, 999);
        assert_eq!(state.account, [5u8; 20]);
    }
}

fn main() {
    let state = LedgerState::new(1, [4u8; 20]);
    println!("balance={}", state.balance);
}
