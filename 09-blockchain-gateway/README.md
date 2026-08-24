# Section 09: Blockchain Gateway

## Goal

Build a unified abstraction that works across Bitcoin, EVM, and Solana without forcing incompatible operations into one interface.

> **Status:** Production-oriented learning model using local adapters; it is not a live multi-chain gateway.

## Why This Section

Now you know three distinct blockchain models. How do you build a system that talks to all three without a tangled mess of `if chain == Bitcoin`?

The answer is **traits** — but done carefully. You'll learn when to unify and when to keep things separate.

## The Challenge

```
Bitcoin: UTXO model (no balance field)
EVM: Account model (always has balance)
Solana: Account + program model (no unified "balance")

How do you query balance across all three?
Answer: You can't with one method. Some chains don't have "balance" as a primitive.
```

## Concepts You'll Learn

| Concept | Why It Matters |
|---------|---|
| Trait design | Capturing common operations |
| Adapter pattern | Chain-specific implementations |
| Avoiding overfit | Not forcing Bitcoin into EVM abstraction |
| Routing | Selecting the right adapter |

## Files You'll Create

1. `01_blockchain_trait.rs` — Common trait definition
2. `02_bitcoin_adapter.rs` — Bitcoin implementation
3. `03_evm_adapter.rs` — EVM implementation
4. `04_solana_adapter.rs` — Solana implementation
5. `05_routing.rs` — Chain routing logic
6. `06_gateway.rs` — Full gateway

## The Design Pattern

```rust
pub trait Blockchain {
    async fn latest_block(&self) -> Result<BlockInfo, Error>;
    async fn get_transaction(&self, hash: &[u8]) -> Result<TxInfo, Error>;
    // ... other common operations
}

impl Blockchain for BitcoinAdapter { ... }
impl Blockchain for EvmAdapter { ... }
impl Blockchain for SolanaAdapter { ... }
```

## Running Tests

```bash
cargo test --package blockchain_gateway
```

## Acceptance Criteria

- [ ] Trait captures truly common operations
- [ ] Each adapter keeps chain-specific details
- [ ] No forced abstractions (e.g., Bitcoin shouldn't fake "balance")
- [ ] Routing works correctly
- [ ] Tests include all three chains
- [ ] Ready for Section 10 (Fund Sweeping)

## Learning Check

- **Rust concepts:** traits, implementations, borrowing, enums, and composition
- **Production problem:** sharing useful gateway behavior without hiding chain-specific differences
- **Simplifications:** adapters return deterministic local values; real RPC transport, error handling, and routing policy are not implemented
- **Exercise:** add a typed error for an unsupported operation instead of forcing every chain to implement it

## Interview Questions

- "Design a trait that abstracts three different blockchains. What methods must it have?"
- "Bitcoin has no 'balance' field. How do you handle this in your gateway?"
- "Should all three adapters have the same error type?"

---

**Next**: Implement `01_blockchain_trait.rs`.
