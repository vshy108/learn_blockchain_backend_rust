// FILE: src/03_reorg_detection.rs
//
// LEARNING OBJECTIVE
// Detect a chain reorg by comparing expected and actual block hashes.
//
// BLOCKCHAIN CONCEPT
// A reorg means a previously indexed block is no longer canonical.
//
// DESIGN DECISION
// Model the detection as a hash comparison.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReorgDetection {
    pub expected_hash: [u8; 32],
    pub actual_hash: [u8; 32],
}

impl ReorgDetection {
    pub fn new(expected_hash: [u8; 32], actual_hash: [u8; 32]) -> Self {
        ReorgDetection {
            expected_hash,
            actual_hash,
        }
    }

    pub fn happened(&self) -> bool {
        self.expected_hash != self.actual_hash
    }
}

fn main() {
    let detection = ReorgDetection::new([1u8; 32], [2u8; 32]);
    println!("Reorg? {}", detection.happened());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_reorg() {
        let detection = ReorgDetection::new([1u8; 32], [2u8; 32]);
        assert!(detection.happened());
    }

    #[test]
    fn no_reorg_when_hash_matches() {
        let detection = ReorgDetection::new([7u8; 32], [7u8; 32]);
        assert!(!detection.happened());
    }
}
