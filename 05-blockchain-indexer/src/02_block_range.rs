// FILE: src/02_block_range.rs
//
// LEARNING OBJECTIVE
// Model a block range for an indexer to process.
//
// BLOCKCHAIN CONCEPT
// A range is a start and end block, often used to resume indexing or catch up.
//
// DESIGN DECISION
// Keep the range model simple and explicit.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockRange {
    pub start: u64,
    pub end: u64,
}

impl BlockRange {
    pub fn new(start: u64, end: u64) -> Self {
        BlockRange { start, end }
    }

    pub fn len(&self) -> u64 {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_range_length() {
        let range = BlockRange::new(100, 106);
        assert_eq!(range.len(), 6);
    }

    #[test]
    fn empty_range_is_detected() {
        assert!(BlockRange::new(10, 10).is_empty());
    }
}

fn main() {
    let range = BlockRange::new(100, 106);
    println!("Range length: {}", range.len());
}
