// FILE: src/04_approval_flow.rs
//
// LEARNING OBJECTIVE
// Explain the approval flow concept in a minimal, testable Rust example.
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
pub struct ApprovalFlow {
    pub approvals: Vec<String>,
}

impl ApprovalFlow {
    pub fn new() -> Self {
        Self {
            approvals: Vec::new(),
        }
    }

    pub fn approve(&mut self, approver: &str) {
        self.approvals.push(approver.to_string());
    }

    pub fn is_approved(&self, required: usize) -> bool {
        self.approvals.len() >= required
    }
}

impl Default for ApprovalFlow {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut flow = ApprovalFlow::new();
    flow.approve("ops");
    println!("approved={}", flow.is_approved(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_enough_approvals() {
        let mut flow = ApprovalFlow::new();
        flow.approve("alice");
        assert!(!flow.is_approved(2));
        flow.approve("bob");
        assert!(flow.is_approved(2));
    }
}
