# Section 01: Blockchain Fundamentals

## Goal

Understand what the backend is actually communicating with: blocks, transactions, confirmations, and the distinction between inclusion and finality.

## Why This Section

Most blockchain backend engineers jump straight to "how do I call an RPC?" without understanding **what blocks and transactions fundamentally are**. This section builds that mental model.

## Concepts You'll Learn

| Concept | Why It Matters |
|---------|---|
| Block | Everything on-chain is organized in blocks |
| Block Height vs Hash | Height can change (reorg), hash cannot |
| Transaction Inclusion | "Is my TX on-chain?" |
| Execution Status | "Did my TX succeed?" (different question) |
| Confirmations | "Is it safe yet?" |
| Finality | "Can it ever change?" |
| Chain Reorg | "Why does my indexer need rollback?" |

## Files You'll Create

1. `01_block.rs` — Block structure, height, hash
2. `02_transaction.rs` — Transactions, inclusion, execution
3. `03_account.rs` — Account model (EVM)
4. `04_address.rs` — Address encoding and checksum
5. `05_confirmation.rs` — Confirmations and safety
6. `06_finality.rs` — Finalized blocks vs confirmed

## Key Distinctions

### Confirmed vs Finalized

**Confirmed (6 blocks deep)**:
```
... → [TX block] → [+1] → [+2] → [+3] → [+4] → [+5] → [+6] ← current
```
- TX is 6 blocks in the past
- Very unlikely to reorg, but possible
- Safe for most user withdrawals

**Finalized (2 epochs on Ethereum)**:
```
... → [TX block] → [validators can NO LONGER reorg] ← current
```
- Mathematically impossible for validators to change (slashing penalty)
- Truly permanent
- Safe for custodial operations

### Inclusion vs Execution

A transaction can be **included** but **not executed successfully**:

```
Transaction broadcast
     ↓
Miner includes in block
     ↓
TX status: SUCCESS (executed correctly)
   OR
TX status: REVERT (execution failed, but still cost gas)
```

**Critical for backend engineers**: A failed transaction is still a transaction. It cost gas. It affected nonce. It appeared on-chain.

## Running Tests

```bash
# Test this section only
cargo test --package 01-blockchain-fundamentals

# Test with output
cargo test --package 01-blockchain-fundamentals -- --nocapture

# Test one file
cargo test --package 01-blockchain-fundamentals block --nocapture
```

## Acceptance Criteria

After completing this section:

- [ ] Can explain what a block is
- [ ] Understand block height, hash, and why they differ
- [ ] Know why "confirmed" ≠ "finalized"
- [ ] Can describe what happens during a chain reorg
- [ ] Can distinguish transaction inclusion from execution
- [ ] Ready for Section 02 (EVM RPC)

## Interview Questions

- "Explain block finality. Why does Ethereum take ~2 minutes to finalize, but Bitcoin's confirmation is practical not mathematical?"
- "What's the difference between a block's height and its hash?"
- "If a block is reorged, what happens to the transactions in it?"
- "A transaction is in a block. Does that mean it succeeded?"

---

**Next**: Read the comments in `01_block.rs`, then implement it.
