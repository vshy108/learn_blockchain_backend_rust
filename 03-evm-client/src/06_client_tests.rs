// FILE: src/06_client_tests.rs
//
// LEARNING OBJECTIVE
// Learn how to test a client layer without relying on a live RPC node.
//
// BLOCKCHAIN CONCEPT
// Client code should be testable through a fake response layer or a mock transport.
// The public API should stay stable even when the underlying transport changes.
//
// NORMAL CASE
// - create a fake response payload
// - ask the client for `latest_block`
// - assert the parsed result is correct
//
// SPECIAL CASES
// - malformed JSON from the node
// - error result object from the node
//
// EXCEPTIONAL CASES
// - network outage
// - timeout or unreachable RPC endpoint
//
// DESIGN DECISION
// This lesson focuses on the testing pattern, not an external HTTP server.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FakeTransport {
    pub response: String,
}

impl FakeTransport {
    pub fn new(response: &str) -> Self {
        FakeTransport {
            response: response.to_string(),
        }
    }

    pub fn parse_latest_block(&self) -> Result<u64, String> {
        let raw = self.response.trim();
        let hex = raw.strip_prefix("0x").unwrap_or(raw);
        if hex.is_empty() || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("invalid block number response".to_string());
        }
        u64::from_str_radix(hex, 16).map_err(|_| "block number out of range".to_string())
    }
}

fn main() {
    let transport = FakeTransport::new("0x15");
    match transport.parse_latest_block() {
        Ok(block_number) => println!("Latest block from fake transport: {block_number}"),
        Err(error) => println!("Could not parse latest block: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_transport_decodes_latest_block() {
        let transport = FakeTransport::new("0x15");
        assert_eq!(transport.parse_latest_block().unwrap(), 21);
    }

    #[test]
    fn fake_transport_rejects_invalid_hex() {
        let transport = FakeTransport::new("0xzz");
        assert!(transport.parse_latest_block().is_err());
    }
}
