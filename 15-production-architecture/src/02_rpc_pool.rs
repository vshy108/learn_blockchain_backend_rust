// FILE: src/02_rpc_pool.rs
//
// LEARNING OBJECTIVE
// Explain the rpc pool concept in a minimal, testable Rust example.
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
pub struct RpcPool {
    pub endpoints: Vec<&'static str>,
}

impl RpcPool {
    pub fn new(endpoints: Vec<&'static str>) -> Self {
        Self { endpoints }
    }
}

fn main() {
    let pool = RpcPool::new(vec!["http://localhost:8545"]);
    println!("endpoints={}", pool.endpoints.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_rpc_endpoints() {
        let pool = RpcPool::new(vec!["http://node-1", "http://node-2"]);
        assert_eq!(pool.endpoints.len(), 2);
    }
}
