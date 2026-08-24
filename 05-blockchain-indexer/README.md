# Section 05: Blockchain Indexer

## Goal

Learn how to robustly process blockchain state and survive crashes through checkpointing.

> **Status:** Production-oriented learning model using local state; it is not a deployable blockchain indexer.

## Why This Section

Blockchain backends must continuously watch the chain for events. A crash at block 5,000,000 should not force you to restart from block 0. This section teaches the critical pattern: **checkpoint and recover**.

## Concepts You'll Learn

| Concept | Why It Matters |
|---------|---|
| Checkpoint | "I've processed up to block N" |
| Recovery | Restart from checkpoint, not from genesis |
| Reorg detection | Detecting when blocks change |
| Reorg rollback | Undoing indexed state |
| Event parsing | Understanding what happened on-chain |

## The Problem

```
Process blocks 0 → 100 → 1000 → 5000000
                                      ↓
                                  💥 CRASH
                                      ↓
Restart: Do we start from 0? (1 week of reprocessing)
         Or from 5000000? (missing the first 5M blocks)
         Or checkpoint 4999950? (safe recovery)
```

## The Solution

```
Before processing block N:
  Save: checkpoint = N-1

If crash happens:
  Restart: Read checkpoint
           Resume from N

During indexing:
  Detect: block_hash[N] != expected
          (chain reorg happened)
  Rollback: Undo state from block N onward
           Resume from last safe checkpoint
```

## Files You'll Create

1. `01_checkpoint.rs` — Save/restore progress
2. `02_block_range.rs` — Processing ranges of blocks
3. `03_reorg_detection.rs` — Detecting block changes
4. `04_reorg_rollback.rs` — Undoing indexed state
5. `05_event_log.rs` — Parsing EVM logs/events
6. `06_indexer_state.rs` — Full indexer coordination

## Key Data Structures

### Checkpoint
```rust
pub struct Checkpoint {
    pub block_number: u64,
    pub block_hash: [u8; 32],
    pub timestamp: u64,
}
```

### Indexed Event
```rust
pub struct IndexedEvent {
    pub block_number: u64,
    pub transaction_hash: [u8; 32],
    pub log_index: u64,
    pub address: Address,
    pub topics: Vec<[u8; 32]>,
    pub data: Vec<u8>,
}
```

## Running Tests

```bash
cargo test --package blockchain_indexer
```

## Acceptance Criteria

- [ ] Can save and restore checkpoint
- [ ] Recovers correctly after crash
- [ ] Detects chain reorg
- [ ] Rolls back indexed state
- [ ] Handles both initial sync and live indexing
- [ ] Tests include crash simulation
- [ ] Ready for Section 06 (Wallet)

## Learning Check

- **Rust concepts:** structs, collections, borrowing, state updates, and failure-focused tests
- **Production problem:** recovering progress and handling chain reorganizations without starting from genesis
- **Simplifications:** persistence, node communication, and rollback storage are modeled locally
- **Exercise:** add a test proving that reprocessing a checkpoint does not duplicate an indexed event

## Interview Questions

- "You detect a reorg: block 1000 changed. How do you rollback?"
- "Your indexer is at block 5M. Node crashes. Where do you resume?"
- "How do you ensure you don't re-process the same event twice?"

---

**Next**: Implement `01_checkpoint.rs`.
