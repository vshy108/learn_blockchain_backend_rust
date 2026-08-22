// FILE: src/04_block_number.rs
//
// LEARNING OBJECTIVE
// Learn the simplest Ethereum RPC call: `eth_blockNumber`.
//
// BLOCKCHAIN CONCEPT
// `eth_blockNumber` returns the latest known block number as a hex string.
//
// NORMAL CASE
// Result: "0x15" means block 21.
//
// SPECIAL CASES
// - Latest block number is a hex-encoded integer
// - If the node is syncing, it may lag behind
//
// EXCEPTIONAL CASES
// - Request fails due to connection or node issue
// - Node returns an error object rather than a result
//
// DESIGN DECISION
// Provide a small interface to decode a block number from an RPC result.
//
// --- IMPLEMENTATION FOLLOWS ---

// FIX: Rust identifiers cannot start with a digit, so `crate::03_hex_encoding` is invalid.
// Keep the lesson self-contained by parsing the hex string directly in this bin.
fn parse_u64_hex(raw: &str) -> Result<u64, String> {
    let cleaned = raw.trim();
    let stripped = cleaned.strip_prefix("0x").unwrap_or(cleaned);
    if stripped.is_empty() {
        return Err("hex value is empty".to_string());
    }
    if !stripped.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("hex value contains non-hex characters".to_string());
    }
    u64::from_str_radix(stripped, 16).map_err(|_| "hex value too large for u64".to_string())
}

pub fn latest_block_number(raw: &str) -> Result<u64, String> {
    parse_u64_hex(raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_latest_block_number() {
        assert_eq!(latest_block_number("0x15").unwrap(), 21);
    }
}

fn main() {
    println!("Latest block: {:?}", latest_block_number("0x15"));
}
