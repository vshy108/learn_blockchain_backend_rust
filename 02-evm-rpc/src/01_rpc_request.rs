// FILE: src/01_rpc_request.rs
//
// LEARNING OBJECTIVE
// Learn how JSON-RPC requests are shaped and why `method`, `params`, and `id` matter.
//
// BLOCKCHAIN CONCEPT
// JSON-RPC 2.0 is the standard protocol used by EVM nodes.
// Every request is a JSON object with a method name and parameters.
//
// NORMAL CASE
// {"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}
//
// SPECIAL CASES
// - Some methods take arrays of params, e.g. `eth_getBalance(address, blockTag)`
// - The `id` field can be integer or string, but must be stable for matching
//
// EXCEPTIONAL CASES
// - Empty params array for no arguments
// - Error responses omit `result`
// - Method not found or invalid params are protocol-level errors
//
// DESIGN DECISION
// Keep the request model simple: a typed structure that serializes to JSON.
//
// --- IMPLEMENTATION FOLLOWS ---

use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<Value>,
    pub id: u64,
}

impl RpcRequest {
    pub fn new(method: &str, params: Vec<Value>, id: u64) -> Self {
        RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id,
        }
    }

    pub fn as_json(&self) -> Value {
        json!({
            "jsonrpc": self.jsonrpc,
            "method": self.method,
            "params": self.params,
            "id": self.id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_eth_block_number_request() {
        let req = RpcRequest::new("eth_blockNumber", Vec::new(), 1);
        let value = req.as_json();

        assert_eq!(value["jsonrpc"], "2.0");
        assert_eq!(value["method"], "eth_blockNumber");
        assert_eq!(value["id"], 1);
    }

    #[test]
    fn supports_params_for_get_balance() {
        let req = RpcRequest::new(
            "eth_getBalance",
            vec![json!("0x1234"), json!("latest")],
            2,
        );

        assert_eq!(req.params.len(), 2);
        assert_eq!(req.params[0], json!("0x1234"));
        assert_eq!(req.params[1], json!("latest"));
    }
}

fn main() {
    let request = RpcRequest::new("eth_blockNumber", Vec::new(), 1);
    println!("RPC request: {}", request.as_json());
}
