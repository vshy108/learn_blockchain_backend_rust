// FILE: src/01_custody_model.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletTier {
    Hot,
    Warm,
    Cold,
}

impl WalletTier {
    pub fn max_daily_limit(&self) -> u64 {
        match self {
            Self::Hot => 10_000,
            Self::Warm => 100_000,
            Self::Cold => 1_000_000,
        }
    }
}

fn main() {
    println!("hot limit={}", WalletTier::Hot.max_daily_limit());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallet_tiers_have_expected_limits() {
        assert_eq!(WalletTier::Hot.max_daily_limit(), 10_000);
        assert_eq!(WalletTier::Cold.max_daily_limit(), 1_000_000);
    }
}
