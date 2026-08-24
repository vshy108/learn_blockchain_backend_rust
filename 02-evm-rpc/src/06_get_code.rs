// FILE: src/06_get_code.rs
//
// LEARNING OBJECTIVE
// Learn how to query deployed bytecode from a contract address.
//
// BLOCKCHAIN CONCEPT
// `eth_getCode` returns the deployed bytecode at an address at a given block.
// It can be empty for EOA addresses or non-deployed addresses.
//
// NORMAL CASE
// "0x60006000..." means the contract has runtime bytecode.
//
// SPECIAL CASES
// - An EOA: `0x`
// - A contract: non-empty bytecode
//
// EXCEPTIONAL CASES
// - Invalid address format
// - Node returns error if the address is malformed
//
// DESIGN DECISION
// Represent the address/code query with simple typed inputs and output text.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeQuery {
    pub address: String,
    pub block_tag: String,
}

impl CodeQuery {
    pub fn new(address: &str, block_tag: &str) -> Self {
        CodeQuery {
            address: address.to_string(),
            block_tag: block_tag.to_string(),
        }
    }

    pub fn is_empty_code(&self, code: &str) -> bool {
        code.trim() == "0x" || code.trim().is_empty()
    }
}

fn main() {
    let query = CodeQuery::new("0x000000000000000000000000000000000000dEaD", "latest");
    println!("Address: {} @ {}", query.address, query.block_tag);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_empty_code() {
        let query = CodeQuery::new("0x1234", "latest");
        assert!(query.is_empty_code("0x"));
        assert!(!query.is_empty_code("0x60006000"));
    }
}
