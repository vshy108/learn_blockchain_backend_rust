// FILE: src/03_risk_policy.rs
//
// LEARNING OBJECTIVE
// Explain the risk policy concept in a minimal, testable Rust example.
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
pub struct RiskPolicy {
    pub max_per_tx: u64,
    pub max_daily: u64,
    pub requires_approval: bool,
}

impl RiskPolicy {
    pub fn new(max_per_tx: u64, max_daily: u64, requires_approval: bool) -> Self {
        Self {
            max_per_tx,
            max_daily,
            requires_approval,
        }
    }

    pub fn allows(&self, amount: u64) -> bool {
        amount <= self.max_per_tx
    }
}

fn main() {
    let policy = RiskPolicy::new(250, 10_000, true);
    println!("{}", policy.allows(250));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enforces_limit() {
        let policy = RiskPolicy::new(1000, 50_000, true);
        assert!(policy.allows(1000));
        assert!(!policy.allows(1001));
    }
}
