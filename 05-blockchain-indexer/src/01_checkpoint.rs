// FILE: src/01_checkpoint.rs
//
// LEARNING OBJECTIVE
// Learn how an indexer saves progress so it can resume after a crash.
//
// BLOCKCHAIN CONCEPT
// A checkpoint stores the latest safe block processed.
//
// NORMAL CASE
// checkpoint = block N-1, resume from block N.
//
// SPECIAL CASES
// - If block N is reorged, restart from last good checkpoint.
//
// DESIGN DECISION
// Simple checkpoint data with restore logic.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checkpoint {
    pub block_number: u64,
    pub block_hash: [u8; 32],
}

impl Checkpoint {
    pub fn new(block_number: u64, block_hash: [u8; 32]) -> Self {
        Checkpoint {
            block_number,
            block_hash,
        }
    }

    pub fn advance(&mut self, next_block: u64, next_hash: [u8; 32]) {
        self.block_number = next_block;
        self.block_hash = next_hash;
    }
}

fn main() {
    let mut checkpoint = Checkpoint::new(100, [1u8; 32]);
    checkpoint.advance(101, [2u8; 32]);
    println!("Checkpoint at block {}", checkpoint.block_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_checkpoint_and_advances() {
        let mut checkpoint = Checkpoint::new(100, [1u8; 32]);
        checkpoint.advance(101, [2u8; 32]);
        assert_eq!(checkpoint.block_number, 101);
        assert_eq!(checkpoint.block_hash, [2u8; 32]);
    }
}
