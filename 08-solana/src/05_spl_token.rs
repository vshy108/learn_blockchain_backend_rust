// FILE: src/05_spl_token.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenMint {
    pub mint_address: [u8; 32],
    pub supply: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenAccount {
    pub owner: [u8; 32],
    pub mint: [u8; 32],
    pub amount: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_token_account() {
        let mint = TokenMint { mint_address: [1u8; 32], supply: 1_000_000 };
        let account = TokenAccount { owner: [2u8; 32], mint: mint.mint_address, amount: 500 }; 
        assert_eq!(account.amount, 500);
        assert_eq!(mint.supply, 1_000_000);
    }
}

fn main() {
    let mint = TokenMint { mint_address: [9u8; 32], supply: 10 };
    println!("mint={:?}", mint.mint_address);
}
