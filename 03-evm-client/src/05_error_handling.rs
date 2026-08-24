// FILE: src/05_error_handling.rs
//
// LEARNING OBJECTIVE
// Learn how client code should surface structured errors rather than ad hoc strings.
//
// BLOCKCHAIN CONCEPT
// RPC and parse errors are different. Clients should model them explicitly.
//
// DESIGN DECISION
// Represent errors using enum variants and capture the root cause.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientError {
    RpcError { code: i64, message: String },
    NetworkError(String),
    ParseError(String),
}

impl ClientError {
    pub fn rpc(code: i64, message: &str) -> Self {
        ClientError::RpcError {
            code,
            message: message.to_string(),
        }
    }
}

fn main() {
    let err = ClientError::rpc(-32000, "node unavailable");
    println!("Error: {:?}", err);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_rpc_error() {
        let err = ClientError::rpc(-32601, "Method not found");
        assert!(matches!(err, ClientError::RpcError { code: -32601, .. }));
    }
}
