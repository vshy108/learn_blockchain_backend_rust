// FILE: src/04_account_state.rs
//
// LEARNING OBJECTIVE
// Represent balance and nonce as typed client data instead of raw RPC strings.
//
// BLOCKCHAIN CONCEPT
// Client methods should return application data, not untyped JSON fragments.
//
// DESIGN DECISION
// Keep the account state model simple and explicit.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountState {
    pub balance: u64,
    pub nonce: u64,
}

impl AccountState {
    pub fn new(balance: u64, nonce: u64) -> Self {
        AccountState { balance, nonce }
    }
}

fn main() {
    let state = AccountState::new(10_000, 7);
    println!("Balance={} nonce={}", state.balance, state.nonce);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_state_from_balance_and_nonce() {
        let state = AccountState::new(10_000, 7);
        assert_eq!(state.balance, 10_000);
        assert_eq!(state.nonce, 7);
    }
}
