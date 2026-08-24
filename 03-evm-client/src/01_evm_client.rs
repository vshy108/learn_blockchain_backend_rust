// FILE: src/01_evm_client.rs
//
// LEARNING OBJECTIVE
// Build the abstraction layer between the app and the raw RPC protocol.
//
// BLOCKCHAIN CONCEPT
// A client wraps JSON-RPC requests and responses, exposing simple typed methods.
//
// NORMAL CASE
// The application asks for `latest_block` or `balance`, not raw JSON.
//
// SPECIAL CASES
// - Some methods are async because they hit the network
// - Error handling distinguishes RPC errors from parsing failures
//
// EXCEPTIONAL CASES
// - Node unreachable
// - Invalid method or malformed params
//
// DESIGN DECISION
// Keep the public API small and type-safe.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvmClient {
    pub rpc_url: String,
}

impl EvmClient {
    pub fn new(rpc_url: &str) -> Self {
        EvmClient {
            rpc_url: rpc_url.to_string(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.rpc_url
    }
}

fn main() {
    let client = EvmClient::new("http://localhost:8545");
    println!("RPC URL: {}", client.base_url());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_client_with_url() {
        let client = EvmClient::new("http://localhost:8545");
        assert_eq!(client.base_url(), "http://localhost:8545");
    }
}
