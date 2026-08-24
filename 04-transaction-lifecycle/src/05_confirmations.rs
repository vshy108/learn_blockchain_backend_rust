// FILE: src/05_confirmations.rs
//
// LEARNING OBJECTIVE
// Count how many block confirmations a transaction has.
//
// BLOCKCHAIN CONCEPT
// Confirmations are the number of blocks mined after the transaction's inclusion block.
//
// NORMAL CASE
// tx in block 100, latest = 106 -> 6 confirmations.
//
// SPECIAL CASES
// - Reorg can reduce confirmations
// - Pending tx has 0 confirmations
//
// DESIGN DECISION
// Keep this as a simple arithmetic model.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmationState {
    pub tx_block: u64,
    pub latest_block: u64,
}

impl ConfirmationState {
    pub fn new(tx_block: u64, latest_block: u64) -> Self {
        ConfirmationState {
            tx_block,
            latest_block,
        }
    }

    pub fn confirmations(&self) -> u64 {
        if self.latest_block <= self.tx_block {
            return 0;
        }
        self.latest_block - self.tx_block
    }
}

fn main() {
    let state = ConfirmationState::new(100, 106);
    println!("Confirmations: {}", state.confirmations());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_confirmations() {
        assert_eq!(ConfirmationState::new(100, 106).confirmations(), 6);
    }

    #[test]
    fn pending_tx_is_zero_confirmations() {
        assert_eq!(ConfirmationState::new(100, 100).confirmations(), 0);
    }
}
