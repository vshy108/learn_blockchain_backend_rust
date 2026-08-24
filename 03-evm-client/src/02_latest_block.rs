// FILE: src/02_latest_block.rs
//
// LEARNING OBJECTIVE
// Expose the latest block number from the client API.
//
// BLOCKCHAIN CONCEPT
// The client should turn a raw RPC result into a typed integer, not leave callers to parse hex.
//
// DESIGN DECISION
// This is a thin wrapper over the block-number parser.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatestBlockResult {
    pub block_number: u64,
}

impl LatestBlockResult {
    pub fn from_hex(raw: &str) -> Result<Self, String> {
        let stripped = raw.trim().strip_prefix("0x").unwrap_or(raw.trim());
        if stripped.is_empty() || !stripped.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("invalid hex block number".to_string());
        }
        let value = u64::from_str_radix(stripped, 16)
            .map_err(|_| "block number out of range".to_string())?;
        Ok(LatestBlockResult {
            block_number: value,
        })
    }
}

fn main() {
    match LatestBlockResult::from_hex("0x15") {
        Ok(result) => println!("Latest block: {}", result.block_number),
        Err(error) => println!("Could not parse latest block: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hex_block_number() {
        let value = LatestBlockResult::from_hex("0x15").unwrap();
        assert_eq!(value.block_number, 21);
    }
}
