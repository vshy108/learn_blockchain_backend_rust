// FILE: src/05_get_block.rs
//
// LEARNING OBJECTIVE
// Learn how to request block details by number or hash with `eth_getBlockByNumber`.
//
// BLOCKCHAIN CONCEPT
// RPC calls can request a specific block and optionally include full transactions.
//
// NORMAL CASE
// `eth_getBlockByNumber("0x1234", true)` returns a block object.
//
// SPECIAL CASES
// - `false` returns only header-level data
// - `true` includes all transactions
//
// EXCEPTIONAL CASES
// - Unknown block number
// - Node not synced
//
// DESIGN DECISION
// Capture a small representation of the block request and the expected response fields.
//
// --- IMPLEMENTATION FOLLOWS ---

use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockRequest {
    pub block_tag: String,
    pub include_txs: bool,
}

impl BlockRequest {
    pub fn new(block_tag: &str, include_txs: bool) -> Self {
        BlockRequest {
            block_tag: block_tag.to_string(),
            include_txs,
        }
    }

    pub fn as_params(&self) -> Vec<Value> {
        vec![json!(self.block_tag), json!(self.include_txs)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_request_params() {
        let req = BlockRequest::new("0x10", true);
        let params = req.as_params();
        assert_eq!(params[0], json!("0x10"));
        assert_eq!(params[1], json!(true));
    }
}

fn main() {
    let req = BlockRequest::new("latest", false);
    println!("{}", serde_json::to_string_pretty(&req.as_params()).unwrap());
}
