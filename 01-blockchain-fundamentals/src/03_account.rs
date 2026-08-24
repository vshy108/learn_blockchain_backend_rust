// FILE: src/03_account.rs
//
// LEARNING OBJECTIVE
// Learn the EVM account model and why it differs from Bitcoin's UTXO model.
//
// BLOCKCHAIN CONCEPT
// In Ethereum and other EVM chains, each address is an account with state:
//   - balance
//   - nonce
//   - code (if contract account)
//   - storage (for contract accounts)
//
// This is not the same as Bitcoin, where funds are represented as spendable outputs.
//
// NORMAL CASE
// - An externally owned account (EOA) has a balance and nonce
// - A contract account has code and storage as well as a balance
// - Transactions update account state deterministically
//
// SPECIAL CASES
// - Contract creation: address starts empty and gains code during deployment
// - Contract call: code executes against storage and may modify state
// - ETH transfer: balance change without contract execution
//
// EXCEPTIONAL CASES
// - Insufficient funds for transfer or gas
// - Nonce mismatch (transaction ordering problem)
// - Reentrant call patterns (contract execution edge cases)
// - Storage collisions / state overwrites
//
// DESIGN DECISION
// We model accounts with the main EVM state fields so the app can reason about:
//   - balance changes
//   - transaction order
//   - contract/state semantics
//
// This helps later when we compare EVM to UTXO and Solana.
//
// --- IMPLEMENTATION FOLLOWS ---

use std::fmt;

/// Ethereum-like account model.
///
/// This is deliberately simplified but consistent with EVM state semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    /// Account address (20-byte EVM address).
    pub address: Address,

    /// Balance in the chain's smallest unit (wei for ETH).
    pub balance: u64,

    /// Nonce for transactions from this account.
    pub nonce: u64,

    /// Optional contract bytecode.
    pub code: Vec<u8>,

    /// Whether this account is a contract account.
    pub is_contract: bool,
}

/// EVM-style address: 20 bytes.
pub type Address = [u8; 20];

impl Account {
    /// Create an externally owned account (EOA).
    pub fn eoa(address: Address, balance: u64, nonce: u64) -> Self {
        Account {
            address,
            balance,
            nonce,
            code: Vec::new(),
            is_contract: false,
        }
    }

    /// Create a contract account.
    pub fn contract(address: Address, balance: u64, nonce: u64, code: Vec<u8>) -> Self {
        Account {
            address,
            balance,
            nonce,
            code,
            is_contract: true,
        }
    }

    /// Deposit funds into the account.
    pub fn credit(&mut self, amount: u64) {
        self.balance = self.balance.saturating_add(amount);
    }

    /// Withdraw funds if sufficient balance exists.
    pub fn debit(&mut self, amount: u64) -> bool {
        if self.balance < amount {
            return false;
        }
        self.balance -= amount;
        true
    }

    /// Increment the nonce for a new outgoing transaction.
    pub fn next_nonce(&mut self) -> u64 {
        self.nonce += 1;
        self.nonce
    }

    /// True if this account can afford a transfer or gas payment.
    pub fn can_afford(&self, amount: u64) -> bool {
        self.balance >= amount
    }

    /// True if this account is a regular wallet account.
    pub fn is_eoa(&self) -> bool {
        !self.is_contract
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Account {}: balance={}, nonce={}, contract={}",
            hex::encode(self.address),
            self.balance,
            self.nonce,
            self.is_contract
        )
    }
}

fn main() {
    let mut wallet = Account::eoa([1u8; 20], 1_000_000, 0);
    println!("Wallet before: {}", wallet);

    wallet.credit(500);
    println!("Wallet after credit: {}", wallet);

    assert!(wallet.debit(200));
    println!("Wallet after debit: {}", wallet);

    println!("Next nonce: {}", wallet.next_nonce());
}

// --- TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    fn address(n: u8) -> Address {
        [n; 20]
    }

    #[test]
    fn eoa_account_is_created_with_balance_and_nonce() {
        let account = Account::eoa(address(1), 1_000_000, 7);

        assert_eq!(account.address, address(1));
        assert_eq!(account.balance, 1_000_000);
        assert_eq!(account.nonce, 7);
        assert!(!account.is_contract);
        assert!(account.is_eoa());
    }

    #[test]
    fn contract_account_has_code() {
        let account = Account::contract(address(2), 500, 12, vec![0x60, 0x00, 0x52]);

        assert!(account.is_contract);
        assert!(!account.is_eoa());
        assert_eq!(account.code, vec![0x60, 0x00, 0x52]);
    }

    #[test]
    fn credit_increases_balance() {
        let mut account = Account::eoa(address(3), 100, 0);

        account.credit(25);

        assert_eq!(account.balance, 125);
    }

    #[test]
    fn debit_requires_sufficient_funds() {
        let mut account = Account::eoa(address(4), 100, 0);

        assert!(account.debit(40));
        assert_eq!(account.balance, 60);
        assert!(!account.debit(100));
        assert_eq!(account.balance, 60);
    }

    #[test]
    fn next_nonce_advances_transaction_order() {
        let mut account = Account::eoa(address(5), 0, 10);

        assert_eq!(account.next_nonce(), 11);
        assert_eq!(account.nonce, 11);
    }

    #[test]
    fn can_afford_checks_balance() {
        let account = Account::eoa(address(6), 42, 0);

        assert!(account.can_afford(42));
        assert!(!account.can_afford(43));
    }
}

// --- RUST VS GO COMPARISON ---
//
// RUST:
// ```rust
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Account {
//     pub address: Address,
//     pub balance: u64,
//     pub nonce: u64,
//     pub code: Vec<u8>,
//     pub is_contract: bool,
// }
// ```
//
// GO:
// ```go
// type Account struct {
//     Address [20]byte
//     Balance uint64
//     Nonce   uint64
//     Code    []byte
//     IsContract bool
// }
// ```
//
// DIFFERENCES:
// - Rust's `Vec<u8>` is explicit and idiomatic for bytes
// - Go uses arrays for addresses and slices for bytes
// - Rust's `derive(PartialEq, Eq)` eliminates manual comparison code
// - Go often uses a struct plus helper methods on top
//
// WHY THIS MATTERS:
// - Account state is the fundamental EVM concept
// - You can see why balance and nonce are separate concerns
// - Contract accounts are different from EOAs, which is critical for security and indexing
