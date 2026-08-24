// FILE: src/01_block.rs
//
// LEARNING OBJECTIVE
// Learn what a block is, why it matters, and distinguish between block height and hash.
//
// BLOCKCHAIN CONCEPT
// A block is an immutable, timestamped collection of transactions linked by cryptographic
// hashes. Every block (except genesis) contains the hash of the prior block, forming a chain.
//
// NORMAL CASE
// When everything works:
//   - A new block arrives every ~12 seconds (Ethereum)
//   - Block contains: height, hash, previous hash, timestamp, transaction list
//   - Each block is immutable once mined
//   - Height is sequential: 0, 1, 2, ...
//   - Hash is computed from block contents via Keccak-256
//
// SPECIAL CASES
// - Block height CAN change if a chain reorg happens (block gets uncled)
// - Block hash CANNOT change (it's the identity of the block)
// - Genesis block has no prior hash (or zeros as prior hash)
// - Empty blocks are valid (no transactions)
//
// EXCEPTIONAL CASES
// - Chain reorg: block at height N gets replaced with a different block at height N
//   (e.g., blocks 100-105 get replaced with 100'-105')
// - RPC node returns stale block (node fell behind)
// - Block not yet finalized (can still be reorged)
//
// DESIGN DECISION
// We use a struct to represent a block with:
//   - height: u64 (block number, can change on reorg)
//   - hash: [u8; 32] (SHA256 for Bitcoin, Keccak256 for EVM; immutable)
//   - previous_hash: [u8; 32] (cryptographic link to prior block)
//   - timestamp: u64 (seconds since epoch)
//   - transaction_count: usize (how many TXs in this block)
//   - miner_reward: u64 (block subsidy + fees; in satoshis/wei/lamports)
//
// Why arrays instead of Vec?
//   - Hash is always 32 bytes; array size is known at compile time
//   - Safer (no allocation, no length mismatch)
//   - More efficient (stack allocation)
//
// Why u64 for height and timestamp?
//   - u64::MAX ~18 billion; blockchain won't hit this in practice
//   - Bitcoin ~850k blocks, Ethereum ~20M blocks (far below u64 limit)
//   - Timestamp: u64 seconds = 584 billion years (well beyond Bitcoin lifespan)
//
// --- IMPLEMENTATION FOLLOWS ---

use sha2::{Digest, Sha256};
use std::fmt;

/// A blockchain block with height, hash, and link to prior block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// Block height (sequential from 0). CAN change on reorg.
    pub height: u64,

    /// Block hash (SHA256 for Bitcoin, Keccak256 for EVM). IMMUTABLE.
    pub hash: [u8; 32],

    /// Hash of the previous block (genesis has [0u8; 32] or special case).
    pub previous_hash: [u8; 32],

    /// Unix timestamp when block was mined.
    pub timestamp: u64,

    /// Number of transactions in this block.
    pub transaction_count: usize,

    /// Block reward (subsidy + fees) in smallest unit (satoshi, wei, lamport).
    pub miner_reward: u64,
}

impl Block {
    /// Create a new block.
    pub fn new(
        height: u64,
        hash: [u8; 32],
        previous_hash: [u8; 32],
        timestamp: u64,
        transaction_count: usize,
        miner_reward: u64,
    ) -> Self {
        Block {
            height,
            hash,
            previous_hash,
            timestamp,
            transaction_count,
            miner_reward,
        }
    }

    /// Check if this is the genesis block (height 0).
    pub fn is_genesis(&self) -> bool {
        self.height == 0
    }

    /// Verify that a potential next block actually links to this block.
    ///
    /// Returns true if:
    ///   - next_block.height == self.height + 1
    ///   - next_block.previous_hash == self.hash
    ///
    /// This is the check that maintains the chain.
    pub fn validates_next(&self, next_block: &Block) -> bool {
        next_block.height == self.height + 1 && next_block.previous_hash == self.hash
    }

    /// Compute a mock block hash (SHA256 of serialized data).
    ///
    /// In real blockchains:
    ///   - Bitcoin: double SHA256 of block header
    ///   - Ethereum: Keccak256 of RLP-encoded block
    ///
    /// This is a simplified version for learning.
    pub fn compute_hash(&self) -> [u8; 32] {
        let data = format!(
            "{}:{}:{}:{}",
            self.height,
            hex::encode(self.previous_hash),
            self.timestamp,
            self.transaction_count
        );
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        // Convert GenericArray to [u8; 32]
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block #{} (hash: {}, prev: {}, ts: {}, txs: {}, reward: {})",
            self.height,
            hex::encode(&self.hash[..8]), // Show first 8 bytes for readability
            hex::encode(&self.previous_hash[..8]),
            self.timestamp,
            self.transaction_count,
            self.miner_reward
        )
    }
}

fn main() {
    // Example: Create and inspect blocks
    let genesis = Block::new(0, [0u8; 32], [0u8; 32], 1692000000, 0, 0);
    println!("Genesis: {}", genesis);

    let block1 = Block::new(1, [1u8; 32], genesis.hash, 1692000012, 5, 50_000_000_000);
    println!("Block 1: {}", block1);

    // Verify chain link
    if genesis.validates_next(&block1) {
        println!("✓ Block 1 correctly links to genesis");
    } else {
        println!("✗ Block 1 does not link to genesis");
    }
}

// --- TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_block_is_identified() {
        let genesis = Block::new(0, [0u8; 32], [0u8; 32], 1000, 0, 50_000_000_000);
        assert!(genesis.is_genesis());
        assert_eq!(genesis.height, 0);
    }

    #[test]
    fn non_genesis_block_is_identified() {
        let block = Block::new(1, [1u8; 32], [0u8; 32], 1012, 5, 50_000_000_000);
        assert!(!block.is_genesis());
        assert_eq!(block.height, 1);
    }

    #[test]
    fn block_validates_correct_next() {
        let block1 = Block::new(1, [1u8; 32], [0u8; 32], 1012, 5, 50_000_000_000);
        let block2 = Block::new(
            2,
            [2u8; 32],
            [1u8; 32], // Points to block1's hash
            1024,
            3,
            50_000_000_000,
        );

        assert!(block1.validates_next(&block2));
    }

    #[test]
    fn block_rejects_wrong_height() {
        let block1 = Block::new(1, [1u8; 32], [0u8; 32], 1012, 5, 50_000_000_000);
        let block_wrong = Block::new(3, [3u8; 32], [1u8; 32], 1024, 3, 50_000_000_000);

        // height should be 2, not 3
        assert!(!block1.validates_next(&block_wrong));
    }

    #[test]
    fn block_rejects_wrong_previous_hash() {
        let block1 = Block::new(1, [1u8; 32], [0u8; 32], 1012, 5, 50_000_000_000);
        let block_wrong = Block::new(
            2,
            [2u8; 32],
            [99u8; 32], // Points to wrong hash
            1024,
            3,
            50_000_000_000,
        );

        assert!(!block1.validates_next(&block_wrong));
    }

    #[test]
    fn compute_hash_is_deterministic() {
        let block = Block::new(1, [0u8; 32], [0u8; 32], 1012, 5, 50_000_000_000);
        let hash1 = block.compute_hash();
        let hash2 = block.compute_hash();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn different_blocks_have_different_hashes() {
        let block1 = Block::new(1, [0u8; 32], [0u8; 32], 1012, 5, 50_000_000_000);
        let block2 = Block::new(2, [0u8; 32], [0u8; 32], 1012, 5, 50_000_000_000);

        assert_ne!(block1.compute_hash(), block2.compute_hash());
    }

    #[test]
    fn hash_changes_if_contents_change() {
        let mut block = Block::new(1, [0u8; 32], [0u8; 32], 1012, 5, 50_000_000_000);
        let hash_before = block.compute_hash();

        block.transaction_count = 10;
        let hash_after = block.compute_hash();

        assert_ne!(hash_before, hash_after);
    }

    #[test]
    fn block_display_shows_key_info() {
        let block = Block::new(100, [255u8; 32], [0u8; 32], 1692345678, 42, 50_000_000_000);
        let display = format!("{}", block);

        assert!(display.contains("Block #100"));
        assert!(display.contains("42")); // tx count
    }
}

// --- RUST VS GO COMPARISON ---
//
// RUST:
// ```rust
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Block {
//     pub height: u64,
//     pub hash: [u8; 32],
//     pub previous_hash: [u8; 32],
//     pub timestamp: u64,
//     pub transaction_count: usize,
//     pub miner_reward: u64,
// }
// ```
//
// GO:
// ```go
// type Block struct {
//     Height           uint64
//     Hash             [32]byte
//     PreviousHash     [32]byte
//     Timestamp        uint64
//     TransactionCount int
//     MinerReward      uint64
// }
// ```
//
// DIFFERENCES:
// - Rust requires explicit derive macros; Go derives automatically
// - Rust: [u8; 32] is a fixed array (stack); Go: [32]byte (also stack)
// - Rust: usize for counts (platform-dependent); Go: int (explicit size)
// - Rust: methods attached via impl; Go: methods attached via receiver
// - Rust: no nullable fields; Go: all fields non-null by default
//
// WHY RUST'S APPROACH MATTERS FOR BLOCKCHAIN:
// - Fixed-size arrays prevent hash length bugs (32 bytes is guaranteed)
// - No implicit nullability (no "hash could be nil" surprises)
// - Derive Debug + Clone + PartialEq for free (Go requires hand-coding)
