// FILE: src/04_address.rs
//
// LEARNING OBJECTIVE
// Learn why blockchain addresses look different from simple UUIDs or usernames.
//
// BLOCKCHAIN CONCEPT
// An address is a stable, usually 20-byte identifier for an account or contract.
// It is often derived from a public key via hashing and case folding.
//
// NORMAL CASE
// - Public key -> Keccak-256 hash -> last 20 bytes -> address
// - Addresses are usually hex strings with 40 characters (without 0x prefix)
// - Ethereum-like addresses are case-insensitive when checking equality
//
// SPECIAL CASES
// - EIP-55 checksum: uppercase/lowercase hints at validity
// - Address can be a contract or externally owned account
// - Zero address is a reserved address: 0x000...000
//
// EXCEPTIONAL CASES
// - Invalid length (not 20 bytes)
// - Mixed-case string that fails checksum
// - Wrong prefix or non-hex characters
//
// DESIGN DECISION
// We implement a small Address type with conversion helpers and checksum validation.
// This keeps the lesson focused on the blockchain semantics rather than full wallet logic.
//
// --- IMPLEMENTATION FOLLOWS ---

use sha3::{Digest, Keccak256};
use std::fmt;

/// 20-byte address used in Ethereum-like systems.
pub type Address = [u8; 20];

/// A minimal Ethereum-style address utility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressUtil;

impl AddressUtil {
    /// Convert a 20-byte address to a lowercase hex string.
    pub fn to_hex(address: &Address) -> String {
        hex::encode(address)
    }

    /// Convert a hex string to a 20-byte address.
    pub fn from_hex(input: &str) -> Result<Address, String> {
        let cleaned = input.trim().trim_start_matches("0x");
        if cleaned.len() != 40 {
            return Err(format!(
                "address must be 40 hex chars, got {}",
                cleaned.len()
            ));
        }
        if !cleaned.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("address contains non-hex characters".to_string());
        }

        let mut bytes = [0u8; 20];
        let decoded = hex::decode(cleaned).map_err(|_| "invalid hex".to_string())?;
        bytes.copy_from_slice(&decoded);
        Ok(bytes)
    }

    /// Compute the 20-byte address from a public key bytes.
    /// Concretely: Keccak256(public_key)[12..]
    pub fn from_public_key(public_key: &[u8]) -> Address {
        let hash = Keccak256::digest(public_key);
        let mut address = [0u8; 20];
        address.copy_from_slice(&hash[12..32]);
        address
    }

    /// Check if an address is the zero address.
    pub fn is_zero(address: &Address) -> bool {
        *address == [0u8; 20]
    }

    /// EIP-55 checksum validation.
    ///
    /// This is a simplified version that ensures a given mixed-case hex string
    /// matches the expected casing derived from the Keccak hash.
    pub fn is_checksum_valid(address: &str) -> bool {
        let cleaned = address.trim().trim_start_matches("0x");
        if cleaned.len() != 40 || !cleaned.chars().all(|c| c.is_ascii_hexdigit()) {
            return false;
        }

        let mut hex = String::new();
        for (idx, ch) in cleaned.chars().enumerate() {
            let c = ch.to_ascii_lowercase();
            let should_upper = match c {
                'a' | 'b' | 'c' | 'd' | 'e' | 'f' => {
                    let hash = Keccak256::digest(cleaned.to_ascii_lowercase().as_bytes());
                    let nibble = hash[idx / 2] >> (if idx % 2 == 0 { 4 } else { 0 }) & 0x0F;
                    nibble >= 8
                }
                _ => false,
            };
            hex.push(if should_upper {
                c.to_ascii_uppercase()
            } else {
                c
            });
        }
        cleaned == hex
    }
}

impl fmt::Display for AddressUtil {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AddressUtil")
    }
}

fn main() {
    match AddressUtil::from_hex("0x00112233445566778899aabbccddeeff00112233") {
        Ok(address) => {
            println!("Address hex: {}", AddressUtil::to_hex(&address));
            println!("Zero address? {}", AddressUtil::is_zero(&address));
        }
        Err(error) => println!("Invalid address: {error}"),
    }
}

// --- TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_address_to_hex() {
        let address = [0x12u8; 20];
        assert_eq!(
            AddressUtil::to_hex(&address),
            "1212121212121212121212121212121212121212"
        );
    }

    #[test]
    fn parses_valid_hex_address() {
        let addr = AddressUtil::from_hex("0x00112233445566778899aabbccddeeff00112233").unwrap();
        assert_eq!(addr.len(), 20);
        assert_eq!(addr[0], 0x00);
        assert_eq!(addr[19], 0x33);
    }

    #[test]
    fn rejects_invalid_length() {
        assert!(AddressUtil::from_hex("0x1234").is_err());
    }

    #[test]
    fn detects_zero_address() {
        assert!(AddressUtil::is_zero(&[0u8; 20]));
        assert!(!AddressUtil::is_zero(&[1u8; 20]));
    }

    #[test]
    fn derives_address_from_public_key() {
        let pubkey = [0xAAu8; 65];
        let address = AddressUtil::from_public_key(&pubkey);
        assert_eq!(address.len(), 20);
        assert_ne!(address, [0u8; 20]);
    }
}
