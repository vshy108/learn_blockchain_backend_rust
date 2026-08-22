// FILE: src/01_private_key.rs
//
// LEARNING OBJECTIVE
// Learn how a wallet starts with a private key and why it must never be printed.
//
// BLOCKCHAIN CONCEPT
// A private key is a 32-byte secret used to sign transactions and derive the public key.
//
// NORMAL CASE
// - random 32-byte value
// - test keys are deterministic for reproducible examples
//
// SPECIAL CASES
// - zero key is invalid in practice
// - very low entropy keys are not used
//
// DESIGN DECISION
// Use a fixed-size array to make secret handling explicit.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateKey {
    pub bytes: [u8; 32],
}

impl PrivateKey {
    pub fn new(bytes: [u8; 32]) -> Self {
        PrivateKey { bytes }
    }

    pub fn is_zero(&self) -> bool {
        self.bytes == [0u8; 32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_private_key() {
        let key = PrivateKey::new([1u8; 32]);
        assert_eq!(key.bytes, [1u8; 32]);
    }

    #[test]
    fn zero_key_is_detected() {
        let key = PrivateKey::new([0u8; 32]);
        assert!(key.is_zero());
    }
}

fn main() {
    let key = PrivateKey::new([1u8; 32]);
    println!("Private key bytes: {:?}", key.bytes);
}
