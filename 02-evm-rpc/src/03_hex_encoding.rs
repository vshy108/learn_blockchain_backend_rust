// FILE: src/03_hex_encoding.rs
//
// LEARNING OBJECTIVE
// Learn how EVM numbers are represented as hex and why `0x` prefix matters.
//
// BLOCKCHAIN CONCEPT
// Ethereum values are usually encoded as hexadecimal strings with the `0x` prefix.
// The string is not the same as a decimal integer; it must be parsed safely.
//
// NORMAL CASE
// - `0x0` = 0
// - `0x10` = 16
// - `0x64` = 100
//
// SPECIAL CASES
// - `0x` alone is invalid
// - Large numbers need `u64`/`u128`/`u256` handling
//
// EXCEPTIONAL CASES
// - Input not hex
// - Input too large for target integer type
//
// DESIGN DECISION
// Provide a minimal hex parser for RPC-oriented values.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexNumber;

impl HexNumber {
    pub fn parse_u64(value: &str) -> Result<u64, String> {
        let cleaned = value.trim();
        let stripped = cleaned.strip_prefix("0x").unwrap_or(cleaned);
        if stripped.is_empty() {
            return Err("hex value is empty".to_string());
        }
        if !stripped.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("hex value contains non-hex characters".to_string());
        }
        u64::from_str_radix(stripped, 16).map_err(|_| "hex value too large for u64".to_string())
    }

    pub fn format_u64(value: u64) -> String {
        format!("0x{:x}", value)
    }
}

fn main() {
    println!("{}", HexNumber::format_u64(255));
    println!("{:?}", HexNumber::parse_u64("0x10"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hex_to_u64() {
        assert_eq!(HexNumber::parse_u64("0x10").unwrap(), 16);
        assert_eq!(HexNumber::parse_u64("0x64").unwrap(), 100);
    }

    #[test]
    fn rejects_invalid_hex() {
        assert!(HexNumber::parse_u64("0xgg").is_err());
    }

    #[test]
    fn formats_u64_to_hex() {
        assert_eq!(HexNumber::format_u64(255), "0xff");
    }
}
