// FILE: src/04_sweep_transaction.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SweepTx {
    pub to: [u8; 20],
    pub value: u64,
}

impl SweepTx {
    pub fn new(to: [u8; 20], value: u64) -> Self {
        Self { to, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_sweep_transaction() {
        let tx = SweepTx::new([5u8; 20], 250);
        assert_eq!(tx.value, 250);
        assert_eq!(tx.to.len(), 20);
    }
}

fn main() {
    let tx = SweepTx::new([9u8; 20], 10);
    println!("value={}", tx.value);
}
