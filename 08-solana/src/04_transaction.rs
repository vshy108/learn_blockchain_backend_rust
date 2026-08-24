// FILE: src/04_transaction.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub program_id: [u8; 32],
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub instructions: Vec<Instruction>,
    pub signatures: Vec<[u8; 64]>,
}

impl Transaction {
    pub fn new(instructions: Vec<Instruction>, signatures: Vec<[u8; 64]>) -> Self {
        Self {
            instructions,
            signatures,
        }
    }
}

fn main() {
    let tx = Transaction::new(vec![], vec![]);
    println!("instructions={}", tx.instructions.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_transaction() {
        let tx = Transaction::new(
            vec![Instruction {
                program_id: [1u8; 32],
                data: vec![0x7f],
            }],
            vec![[2u8; 64]],
        );
        assert_eq!(tx.instructions.len(), 1);
        assert_eq!(tx.signatures.len(), 1);
    }
}
