// FILE: src/01_sweep_detector.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deposit {
    pub wallet: [u8; 20],
    pub amount: u64,
}

pub fn detect_sweepable(deposits: &[Deposit], threshold: u64) -> Vec<Deposit> {
    deposits
        .iter()
        .filter(|d| d.amount >= threshold)
        .cloned()
        .collect()
}

fn main() {
    println!("{}", detect_sweepable(&[], 10).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_only_large_enough_deposits() {
        let deposits = [
            Deposit {
                wallet: [1u8; 20],
                amount: 100,
            },
            Deposit {
                wallet: [2u8; 20],
                amount: 500,
            },
        ];
        let sweepable = detect_sweepable(&deposits, 250);
        assert_eq!(sweepable.len(), 1);
        assert_eq!(sweepable[0].wallet, [2u8; 20]);
    }
}
