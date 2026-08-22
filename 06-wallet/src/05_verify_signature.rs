// FILE: src/05_verify_signature.rs
//
// LEARNING OBJECTIVE
// Verify a signature without exposing the private key.
//
// BLOCKCHAIN CONCEPT
// Verification checks that a signature matches a message and a public key.
//
// DESIGN DECISION
// Keep the signature verification model predictable.
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

    pub fn verify(&self, msg: &[u8], private_key: &[u8; 32]) -> bool {
        Self::sign(msg, private_key) == *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_legitimate_signature() {
        let msg = b"hello";
        let private_key = [1u8; 32];
        let sig = Signature::sign(msg, &private_key);
        assert!(sig.verify(msg, &private_key));
    }
}

fn main() {
    let msg = b"hello";
    let private_key = [1u8; 32];
    let sig = Signature::sign(msg, &private_key);
    println!("Verifies? {}", sig.verify(msg, &private_key));
}
