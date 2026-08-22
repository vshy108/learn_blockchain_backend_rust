// FILE: src/04_reorg_rollback.rs
//
// LEARNING OBJECTIVE
// Model rollback after a detected reorg.
//
// BLOCKCHAIN CONCEPT
// Once a reorg is detected, the indexer should restore the last trusted checkpoint and replay from there.
//
// DESIGN DECISION
// Keep rollback logic explicit and simple.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReorgRollback {
    pub last_safe_block: u64,
    pub to_block: u64,
}

impl ReorgRollback {
    pub fn new(last_safe_block: u64, to_block: u64) -> Self {
        ReorgRollback { last_safe_block, to_block }
    }

    pub fn rollback_to(&self) -> u64 {
        self.last_safe_block
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rollback_returns_last_safe_height() {
        let rollback = ReorgRollback::new(100, 120);
        assert_eq!(rollback.rollback_to(), 100);
    }
}

fn main() {
    let rollback = ReorgRollback::new(100, 120);
    println!("Rollback to block {}", rollback.rollback_to());
}
