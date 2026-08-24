// FILE: src/03_instruction.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub program_id: [u8; 32],
    pub accounts: Vec<[u8; 32]>,
    pub data: Vec<u8>,
}

impl Instruction {
    pub fn new(program_id: [u8; 32], accounts: Vec<[u8; 32]>, data: Vec<u8>) -> Self {
        Self {
            program_id,
            accounts,
            data,
        }
    }
}

fn main() {
    let ix = Instruction::new([5u8; 32], vec![], vec![1]);
    println!("instruction bytes={}", ix.data.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_instruction() {
        let ix = Instruction::new([1u8; 32], vec![[2u8; 32]], vec![0x01, 0x02]);
        assert_eq!(ix.program_id, [1u8; 32]);
        assert_eq!(ix.accounts.len(), 1);
    }
}
