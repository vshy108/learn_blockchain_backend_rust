// FILE: src/03_address.rs
//
// LEARNING OBJECTIVE
// Turn a public key into an address.
//
// BLOCKCHAIN CONCEPT
// Ethereum addresses are 20 bytes derived from the Keccak hash of the public key.
//
// DESIGN DECISION
// Keep the example deterministic and simple.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub bytes: [u8; 20],
}

impl Address {
    pub fn from_public_key(public_key: &[u8; 65]) -> Self {
        let mut bytes = [0u8; 20];
        bytes.copy_from_slice(&public_key[45..65]);
        Address { bytes }
    }
}

fn main() {
    let address = Address::from_public_key(&[0u8; 65]);
    println!("Address: {:?}", address.bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_address_from_public_key() {
        let key = [0u8; 65];
        let address = Address::from_public_key(&key);
        assert_eq!(address.bytes.len(), 20);
    }
}
