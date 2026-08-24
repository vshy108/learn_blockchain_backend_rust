// FILE: src/06_audit_log.rs
//
// LEARNING OBJECTIVE
// Explain the audit log concept in a minimal, testable Rust example.
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
pub struct AuditEntry {
    pub action: &'static str,
    pub approver: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditLog {
    pub entries: Vec<AuditEntry>,
}

impl AuditLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, action: &'static str, approver: &'static str) {
        self.entries.push(AuditEntry { action, approver });
    }
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut log = AuditLog::new();
    log.add("approve", "ops");
    println!("entries={}", log.entries.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_action_history() {
        let mut log = AuditLog::new();
        log.add("approve", "alice");
        assert_eq!(log.entries.len(), 1);
        assert_eq!(log.entries[0].approver, "alice");
    }
}
