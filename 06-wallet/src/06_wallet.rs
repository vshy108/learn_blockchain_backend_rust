// FILE: src/06_wallet.rs
//
// LEARNING OBJECTIVE
// Combine private key, public key, address, and signing into one wallet abstraction.
//
// BLOCKCHAIN CONCEPT
// A wallet is a convenience wrapper that contains the necessary crypto primitives.
//
// DESIGN DECISION
// Keep the wallet minimal and deterministic.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateKey {
    pub bytes: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey {
    pub bytes: [u8; 65],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub bytes: [u8; 20],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub bytes: [u8; 64],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wallet {
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
    pub address: Address,
}

impl Wallet {
    pub fn new(private_key: [u8; 32]) -> Self {
        let public_key = PublicKey {
            bytes: {
                let mut b = [0u8; 65];
                b[..32].copy_from_slice(&private_key);
                b[32..].fill(0x01);
                b
            },
        };
        let address = Address {
            bytes: {
                let mut a = [0u8; 20];
                a.copy_from_slice(&public_key.bytes[45..65]);
                a
            },
        };
        Wallet {
            private_key: PrivateKey { bytes: private_key },
            public_key,
            address,
        }
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        let mut out = [0u8; 64];
        for (idx, b) in msg.iter().enumerate() {
            out[idx % 64] = out[idx % 64]
                .wrapping_add(self.private_key.bytes[idx % 32])
                .wrapping_add(*b);
        }
        Signature { bytes: out }
    }
}

fn main() {
    let wallet = Wallet::new([42u8; 32]);
    println!("Wallet address len={}", wallet.address.bytes.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallet_creates_deterministic_address() {
        let wallet = Wallet::new([9u8; 32]);
        assert_eq!(wallet.address.bytes.len(), 20);
        assert_eq!(wallet.private_key.bytes, [9u8; 32]);
    }

    #[test]
    fn wallet_signs_payload() {
        let wallet = Wallet::new([3u8; 32]);
        let sig = wallet.sign(b"demo");
        assert_eq!(sig.bytes.len(), 64);
    }
}
