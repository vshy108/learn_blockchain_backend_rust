// FILE: src/05_confirmation.rs
//
// LEARNING OBJECTIVE
// Learn how confirmation depth indicates safety and why it matters for wallet UX.
//
// BLOCKCHAIN CONCEPT
// A confirmation is a block mined on top of the transaction's inclusion block.
// If a transaction is included in block 100 and we are now at block 106, it has 6 confirmations.
//
// NORMAL CASE
// - 0 confirmations: pending or unconfirmed
// - 1 confirmation: mined once
// - 6 confirmations: usually considered safe
//
// SPECIAL CASES
// - Reorg can reduce confirmation count
// - If a transaction is in an uncle block, it might be dropped
// - Some chains use different finality thresholds
//
// EXCEPTIONAL CASES
// - RPC report is stale
// - Pending tx still in mempool, not included yet
// - Block height advances without transaction becoming final
//
// DESIGN DECISION
// We model confirmations as a simple block-distance calculation.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmationState {
    pub tx_block_height: u64,
    pub latest_block_height: u64,
}

impl ConfirmationState {
    pub fn new(tx_block_height: u64, latest_block_height: u64) -> Self {
        ConfirmationState {
            tx_block_height,
            latest_block_height,
        }
    }

    pub fn confirmations(&self) -> u64 {
        if self.latest_block_height <= self.tx_block_height {
            return 0;
        }
        self.latest_block_height - self.tx_block_height
    }

    pub fn is_safe(&self, threshold: u64) -> bool {
        self.confirmations() >= threshold
    }
}

fn main() {
    let state = ConfirmationState::new(100, 106);
    println!("Confirmations: {}", state.confirmations());
    println!("Safe at 6? {}", state.is_safe(6));
}

// --- TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_transaction_has_zero_confirmations() {
        let state = ConfirmationState::new(100, 100);
        assert_eq!(state.confirmations(), 0);
    }

    #[test]
    fn mined_transaction_counts_depth() {
        let state = ConfirmationState::new(100, 106);
        assert_eq!(state.confirmations(), 6);
    }

    #[test]
    fn threshold_check_works() {
        let state = ConfirmationState::new(100, 106);
        assert!(state.is_safe(6));
        assert!(!state.is_safe(7));
    }
}
