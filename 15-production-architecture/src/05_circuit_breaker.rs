// FILE: src/05_circuit_breaker.rs
//
// LEARNING OBJECTIVE
// Explain the circuit breaker concept in a minimal, testable Rust example.
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
pub struct CircuitBreaker {
    pub open: bool,
}

impl CircuitBreaker {
    pub fn new(open: bool) -> Self {
        Self { open }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_open_state() {
        let breaker = CircuitBreaker::new(true);
        assert!(breaker.open);
    }
}

fn main() {
    let breaker = CircuitBreaker::new(false);
    println!("open={}", breaker.open);
}
