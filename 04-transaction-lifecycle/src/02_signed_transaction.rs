// FILE: src/02_signed_transaction.rs
//
// LEARNING OBJECTIVE
// Distinguish a transaction payload from a signed transaction that is ready to submit.
//
// BLOCKCHAIN CONCEPT
// Signing proves ownership but the unsigned payload is still the same transaction data.
//
// DESIGN DECISION
// A signed transaction is a wrapper around the payload plus a signature.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedTransaction {
    pub payload: Transaction,
    pub signature: [u8; 64],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub hash: [u8; 32],
    pub from: [u8; 20],
    pub to: Option<[u8; 20]>,
    pub value: u64,
    pub nonce: u64,
    pub gas_limit: u64,
    pub gas_price: u64,
}

impl SignedTransaction {
    pub fn new(payload: Transaction, signature: [u8; 64]) -> Self {
        SignedTransaction { payload, signature }
    }
}

fn main() {
    let payload = Transaction {
        hash: [1u8; 32],
        from: [2u8; 20],
        to: Some([3u8; 20]),
        value: 5,
        nonce: 2,
        gas_limit: 21_000,
        gas_price: 1_000,
    };
    let signed = SignedTransaction::new(payload, [9u8; 64]);
    println!("Signed tx signature len={}", signed.signature.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_a_signed_transaction() {
        let payload = Transaction {
            hash: [1u8; 32],
            from: [2u8; 20],
            to: Some([3u8; 20]),
            value: 5,
            nonce: 2,
            gas_limit: 21_000,
            gas_price: 1_000,
        };
        let signed = SignedTransaction::new(payload.clone(), [9u8; 64]);
        assert_eq!(signed.payload, payload);
        assert_eq!(signed.signature, [9u8; 64]);
    }
}
