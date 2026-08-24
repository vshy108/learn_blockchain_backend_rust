// FILE: src/03_get_block_details.rs
//
// LEARNING OBJECTIVE
// Define the client-facing block details response.
//
// BLOCKCHAIN CONCEPT
// Application code should not handle raw JSON object maps everywhere.
//
// DESIGN DECISION
// Provide a simple block-record shape for the client layer.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockDetails {
    pub number: u64,
    pub hash: String,
    pub parent_hash: String,
}

impl BlockDetails {
    pub fn new(number: u64, hash: &str, parent_hash: &str) -> Self {
        BlockDetails {
            number,
            hash: hash.to_string(),
            parent_hash: parent_hash.to_string(),
        }
    }
}

fn main() {
    let block = BlockDetails::new(123, "0xabc", "0xdef");
    println!(
        "Block #{} hash={} parent={}",
        block.number, block.hash, block.parent_hash
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_block_details() {
        let block = BlockDetails::new(123, "0xabc", "0xdef");
        assert_eq!(block.number, 123);
        assert_eq!(block.hash, "0xabc");
    }
}
