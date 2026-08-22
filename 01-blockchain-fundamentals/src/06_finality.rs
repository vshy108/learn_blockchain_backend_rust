// FILE: src/06_finality.rs
//
// LEARNING OBJECTIVE
// Understand the difference between a transaction being confirmed and being final.
//
// BLOCKCHAIN CONCEPT
// Confirmation depth is a probabilistic safety indicator.
// Finality is a stronger guarantee: the chain cannot be reorganized past this point.
//
// NORMAL CASE
// - A transaction is confirmed once it is in a recent block
// - It becomes finalized once the protocol rules make reorg impossible
//
// SPECIAL CASES
// - Ethereum finality requires validators to attest and a supermajority to finalize
// - Bitcoin finality is probabilistic: more confirms = safer
//
// EXCEPTIONAL CASES
// - Chain reorgs can revert confirmed state
// - Finality can fail under validator misbehavior or network partition
//
// DESIGN DECISION
// We model "finalized" as a boolean flag, while confirmations track safety depth.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalityState {
    pub block_height: u64,
    pub finalized_height: Option<u64>,
}

impl FinalityState {
    pub fn new(block_height: u64, finalized_height: Option<u64>) -> Self {
        FinalityState {
            block_height,
            finalized_height,
        }
    }

    pub fn is_finalized(&self) -> bool {
        self.finalized_height.is_some_and(|height| height >= self.block_height)
    }

    pub fn finality_gap(&self) -> Option<u64> {
        match self.finalized_height {
            Some(finalized) if finalized <= self.block_height => Some(self.block_height - finalized),
            _ => None,
        }
    }
}

// --- TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finalized_block_is_marked_final() {
        let state = FinalityState::new(200, Some(200));
        assert!(state.is_finalized());
    }

    #[test]
    fn unfinalized_block_is_not_final() {
        let state = FinalityState::new(200, Some(199));
        assert!(!state.is_finalized());
    }

    #[test]
    fn gap_reports_distance_to_finalized_height() {
        let state = FinalityState::new(200, Some(197));
        assert_eq!(state.finality_gap(), Some(3));
    }
}

fn main() {
    let state = FinalityState::new(200, Some(200));
    println!("Finalized? {}", state.is_finalized());
    println!("Gap: {:?}", state.finality_gap());
}
