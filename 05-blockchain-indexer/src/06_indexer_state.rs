// FILE: src/06_indexer_state.rs
//
// LEARNING OBJECTIVE
// Bring checkpointing, ranges, and rollback together in a simple indexer state model.
//
// BLOCKCHAIN CONCEPT
// A real indexer keeps track of the last safe block and processing status.
//
// DESIGN DECISION
// Keep the model simple and deterministic.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexerState {
    pub latest_processed_block: u64,
    pub safe_checkpoint: u64,
    pub last_reorg_detected: bool,
}

impl IndexerState {
    pub fn new(latest_processed_block: u64, safe_checkpoint: u64) -> Self {
        IndexerState { latest_processed_block, safe_checkpoint, last_reorg_detected: false }
    }

    pub fn mark_reorg(&mut self) {
        self.last_reorg_detected = true;
        self.latest_processed_block = self.safe_checkpoint;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reorg_resets_to_safe_checkpoint() {
        let mut state = IndexerState::new(200, 150);
        state.mark_reorg();
        assert!(state.last_reorg_detected);
        assert_eq!(state.latest_processed_block, 150);
    }
}

fn main() {
    let mut state = IndexerState::new(200, 150);
    state.mark_reorg();
    println!("Indexer state: {:?}", state);
}
