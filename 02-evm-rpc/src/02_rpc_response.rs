// FILE: src/02_rpc_response.rs
//
// LEARNING OBJECTIVE
// Learn how to distinguish success and error JSON-RPC responses.
//
// BLOCKCHAIN CONCEPT
// Responses are either result-bearing or error-bearing. A successful response includes `result`,
// while an error response includes `error` and no result.
//
// NORMAL CASE
// {"jsonrpc":"2.0","result":"0x1","id":1}
//
// SPECIAL CASES
// - result may be null
// - error object contains code and message
//
// EXCEPTIONAL CASES
// - method not found
// - invalid params
// - execution error in the node itself
//
// DESIGN DECISION
// Represent the response as a typed enum: `Success` or `Error`.
//
// --- IMPLEMENTATION FOLLOWS ---

use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RpcResponse {
    Success { id: u64, result: Value },
    Error { id: u64, code: i64, message: String },
}

impl RpcResponse {
    pub fn from_json(value: &Value) -> Result<Self, String> {
        let id = value["id"].as_u64().ok_or("missing or invalid id")?;

        if let Some(error) = value.get("error") {
            let code = error["code"].as_i64().ok_or("missing error code")?;
            let message = error["message"].as_str().unwrap_or("unknown error").to_string();
            return Ok(RpcResponse::Error { id, code, message });
        }

        let result = value.get("result").cloned().unwrap_or(Value::Null);
        Ok(RpcResponse::Success { id, result })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_success_response() {
        let value = json!({ "jsonrpc": "2.0", "result": "0x1", "id": 1 });
        let response = RpcResponse::from_json(&value).unwrap();

        assert!(matches!(response, RpcResponse::Success { id: 1, .. }));
    }

    #[test]
    fn parses_error_response() {
        let value = json!({
            "jsonrpc": "2.0",
            "error": { "code": -32601, "message": "Method not found" },
            "id": 1
        });

        let response = RpcResponse::from_json(&value).unwrap();
        assert!(matches!(response, RpcResponse::Error { code: -32601, .. }));
    }
}

fn main() {
    let value = json!({ "jsonrpc": "2.0", "result": "0x1", "id": 1 });
    println!("Parsed: {:?}", RpcResponse::from_json(&value));
}
