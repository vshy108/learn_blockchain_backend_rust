// FILE: src/01_transaction_struct.rs
//
// LEARNING OBJECTIVE
// Model a transaction as data, not just a JSON blob.
//
// BLOCKCHAIN CONCEPT
// A transaction carries value, sender, recipient, nonce, and gas fields.
//
// DESIGN DECISION
// Keep the fields explicit and typed.
//
// --- IMPLEMENTATION FOLLOWS ---

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

impl Transaction {
    pub fn new(
        hash: [u8; 32],
        from: [u8; 20],
        to: Option<[u8; 20]>,
        value: u64,
        nonce: u64,
        gas_limit: u64,
        gas_price: u64,
    ) -> Self {
        Transaction {
            hash,
            from,
            to,
            value,
            nonce,
            gas_limit,
            gas_price,
        }
    }

    pub fn total_cost(&self) -> u64 {
        self.value
            .saturating_add(self.gas_limit.saturating_mul(self.gas_price))
    }
}

fn main() {
    let tx = Transaction::new([1u8; 32], [2u8; 20], Some([3u8; 20]), 10, 0, 21_000, 1_000);
    println!("Total cost: {}", tx.total_cost());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_transaction() {
        let tx = Transaction::new([1u8; 32], [2u8; 20], Some([3u8; 20]), 10, 7, 21_000, 1_000);
        assert_eq!(tx.value, 10);
        assert_eq!(tx.nonce, 7);
    }

    #[test]
    fn computes_total_cost() {
        let tx = Transaction::new([1u8; 32], [2u8; 20], Some([3u8; 20]), 10, 0, 21_000, 1_000);
        assert_eq!(tx.total_cost(), 21_000_010);
    }
}
