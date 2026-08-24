// FILE: src/06_transaction_state.rs
//
// LEARNING OBJECTIVE
// Model the lifecycle of a transaction from pending to finalized.
//
// BLOCKCHAIN CONCEPT
// Transactions move through a state machine: pending -> mined -> confirmed -> finalized.
//
// NORMAL CASE
// Broadcast -> included -> confirmed -> finalized.
//
// SPECIAL CASES
// - Reverted tx is mined but failed
// - Dropped tx never reaches mined state
//
// DESIGN DECISION
// Keep the state machine clear and explicitly enumerable.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxState {
    Pending,
    Mined,
    Confirmed,
    Finalized,
    Reverted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionState {
    pub state: TxState,
    pub confirmations: u64,
}

impl TransactionState {
    pub fn new(state: TxState, confirmations: u64) -> Self {
        TransactionState {
            state,
            confirmations,
        }
    }

    pub fn is_final(&self) -> bool {
        self.state == TxState::Finalized
    }
}

fn main() {
    let state = TransactionState::new(TxState::Confirmed, 6);
    println!(
        "State: {:?}, confirmations={}",
        state.state, state.confirmations
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_state_tracks_finality() {
        let state = TransactionState::new(TxState::Finalized, 12);
        assert!(state.is_final());
        assert_eq!(state.confirmations, 12);
    }

    #[test]
    fn reverted_transaction_is_not_final() {
        let state = TransactionState::new(TxState::Reverted, 0);
        assert!(!state.is_final());
    }
}
