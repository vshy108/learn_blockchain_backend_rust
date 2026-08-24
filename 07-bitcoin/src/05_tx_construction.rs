// FILE: src/05_tx_construction.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    pub prev_tx: [u8; 32],
    pub index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
    pub amount: u64,
    pub to: [u8; 20],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

impl Transaction {
    pub fn new(inputs: Vec<Input>, outputs: Vec<Output>) -> Self {
        Self { inputs, outputs }
    }

    pub fn total_amount(&self) -> u64 {
        self.outputs.iter().map(|o| o.amount).sum()
    }
}

fn main() {
    let tx = Transaction::new(vec![], vec![]);
    println!("outputs={}", tx.outputs.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_transaction_from_inputs_and_outputs() {
        let tx = Transaction::new(
            vec![Input {
                prev_tx: [1u8; 32],
                index: 0,
            }],
            vec![
                Output {
                    amount: 75_000,
                    to: [2u8; 20],
                },
                Output {
                    amount: 25_000,
                    to: [3u8; 20],
                },
            ],
        );
        assert_eq!(tx.total_amount(), 100_000);
    }
}
