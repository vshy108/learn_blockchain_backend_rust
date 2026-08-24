# Section 04: Transaction Lifecycle

## Goal

Understand transaction states from creation through finality. Learn why "mined" ≠ "succeeded".

> **Status:** Learning model of transaction lifecycle states; it is not a chain-compatible transaction processor.

## Why This Section

Many backend engineers assume: "if TX is in a block, it worked." This section corrects that critical misunderstanding. A transaction can be included but reverted, consuming gas with no effect.

## Concepts You'll Learn

| Concept | Why It Matters |
|---------|---|
| Transaction creation | Signing, serialization |
| Broadcasting | Propagation through mempool |
| Mined | Included in a block |
| Receipt | Result of execution |
| Status | Success (1) vs Revert (0) |
| State machine | Preventing invalid transitions |

## The State Machine

```
Created → Signed → Broadcast → Pending → Mined → Confirmed → Finalized
                                                  
                                         May revert (status=0)
                                         May be dropped (timeout)
                                         May be replaced (nonce conflict)
```

## Files You'll Create

1. `01_transaction_struct.rs` — Transaction data structure
2. `02_signed_transaction.rs` — Signing and serialization
3. `03_transaction_receipt.rs` — Receipt parsing
4. `04_execution_status.rs` — Success vs revert distinction
5. `05_confirmations.rs` — Counting blocks since inclusion
6. `06_transaction_state.rs` — State machine implementation

## Key Distinctions

### Mined vs Succeeded
```
TX in block (mined):  ✓
TX executed success:  ? (check receipt.status)
```

### Receipt Structure
```rust
pub struct Receipt {
    pub transaction_hash: [u8; 32],
    pub status: u8,  // 1 = success, 0 = revert
    pub gas_used: u64,
    pub cumulative_gas_used: u64,
    pub logs: Vec<Log>,
    pub contract_address: Option<Address>,  // if contract creation
}
```

### Confirmations
- **Confirmation 1**: TX is in latest block
- **Confirmation 6**: TX is 6 blocks in the past
- **Confirmation = 0**: TX not yet mined

## Running Tests

```bash
cargo test --package transaction_lifecycle
```

## Acceptance Criteria

- [ ] Distinguish mined from successful
- [ ] Receipt parsing handles all EVM fields
- [ ] State machine prevents invalid transitions
- [ ] Can compute confirmation count accurately
- [ ] Handle transaction replacement (nonce reuse)
- [ ] Ready for Section 05 (Indexer)

## Learning Check

- **Rust concepts:** enums, `Option`, state transitions, ownership, and invariant-focused tests
- **Production problem:** distinguishing inclusion, execution result, confirmation, and finality
- **Simplifications:** signing, receipts, and chain state are represented with local structs rather than real protocol data
- **Exercise:** add a test proving that a reverted transaction still consumes gas and its nonce

## Interview Questions

- "A transaction is in a block. What else do you need to check?"
- "If a transaction reverts, did it cost gas?"
- "What's the difference between `transactionIndex` and `logIndex` in a receipt?"

---

**Next**: Implement `01_transaction_struct.rs`.
