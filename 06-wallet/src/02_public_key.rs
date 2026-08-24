// FILE: src/02_public_key.rs
//
// LEARNING OBJECTIVE
// Derive the public key from a private key.
//
// BLOCKCHAIN CONCEPT
// Public key derivation uses elliptic curve multiplication; we model the result as bytes.
//
// DESIGN DECISION
// Keep the example simple and deterministic.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey {
    pub bytes: [u8; 65],
}

impl PublicKey {
    pub fn from_private_key(private_key: &[u8; 32]) -> Self {
        let mut bytes = [0u8; 65];
        bytes[..32].copy_from_slice(private_key);
        bytes[32..].fill(0x01);
        PublicKey { bytes }
    }
}

fn main() {
    let key = PublicKey::from_private_key(&[7u8; 32]);
    println!("Public key length: {}", key.bytes.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_public_key_from_private_key() {
        let key = PublicKey::from_private_key(&[7u8; 32]);
        assert_eq!(key.bytes[32], 0x01);
    }
}
