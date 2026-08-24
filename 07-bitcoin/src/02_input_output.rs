// FILE: src/02_input_output.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outpoint {
    pub tx_id: [u8; 32],
    pub index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    pub outpoint: Outpoint,
    pub script_sig: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
    pub amount: u64,
    pub script_pubkey: Vec<u8>,
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

    pub fn total_input(&self) -> u64 {
        self.inputs
            .iter()
            .map(|input| match input.outpoint.index {
                0 => 1000,
                _ => 0,
            })
            .sum()
    }

    pub fn total_output(&self) -> u64 {
        self.outputs.iter().map(|output| output.amount).sum()
    }
}

fn main() {
    let tx = Transaction::new(vec![], vec![]);
    println!("inputs={}, outputs={}", tx.inputs.len(), tx.outputs.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_transaction_with_inputs_and_outputs() {
        let tx = Transaction::new(
            vec![Input {
                outpoint: Outpoint {
                    tx_id: [1u8; 32],
                    index: 0,
                },
                script_sig: vec![0x01],
            }],
            vec![Output {
                amount: 900,
                script_pubkey: vec![0x02],
            }],
        );
        assert_eq!(tx.inputs.len(), 1);
        assert_eq!(tx.outputs.len(), 1);
        assert_eq!(tx.total_output(), 900);
    }
}
