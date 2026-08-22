// FILE: src/01_account.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub pubkey: [u8; 32],
    pub lamports: u64,
    pub owner: [u8; 32],
    pub executable: bool,
}

impl Account {
    pub fn new(pubkey: [u8; 32], lamports: u64, owner: [u8; 32], executable: bool) -> Self {
        Self { pubkey, lamports, owner, executable }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_solana_account() {
        let account = Account::new([1u8; 32], 1_000_000, [2u8; 32], false);
        assert_eq!(account.lamports, 1_000_000);
        assert_eq!(account.owner, [2u8; 32]);
    }
}

fn main() {
    let account = Account::new([9u8; 32], 42, [4u8; 32], false);
    println!("lamports={}", account.lamports);
}
