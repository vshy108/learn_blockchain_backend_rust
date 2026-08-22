// FILE: src/04_signing.rs
//
// LEARNING OBJECTIVE
// Learn the idea of signing data with a private key.
//
// BLOCKCHAIN CONCEPT
// A signature proves the data was authorized by the holder of the private key.
//
// DESIGN DECISION
// Keep the signature as bytes for clarity.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub bytes: [u8; 64],
}

impl Signature {
    pub fn sign(msg: &[u8], private_key: &[u8; 32]) -> Self {
        let mut out = [0u8; 64];
        let mut idx = 0;
        for b in msg {
            out[idx % 64] = out[idx % 64].wrapping_add(private_key[idx % 32]).wrapping_add(*b);
            idx += 1;
        }
        Signature { bytes: out }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signs_payload_with_private_key() {
        let sig = Signature::sign(b"hello", &[1u8; 32]);
        assert_eq!(sig.bytes.len(), 64);
    }
}

fn main() {
    let sig = Signature::sign(b"hello", &[1u8; 32]);
    println!("Signature: {:?}", sig.bytes);
}
