# learn_blockchain_backend

A structured, Rust-first learning journey through blockchain backend engineering. This repository builds a complete blockchain gateway from first principles, teaching both fundamentals and production-grade systems.

## Repo Metadata

- **Started**: 2026-08-22
- **Language**: Rust (1.70+)
- **Focus**: Blockchain backend engineering for exchange/payment systems
- **Structure**: 15 progressive lessons, each building on prior knowledge
- **Time commitment**: Designed for deep learning (4-6 weeks, 1-2 hours/day)

## Why This Repo Matters

Blockchain engineering requires understanding **three distinct mental models** (Bitcoin's UTXO, EVM's account model, Solana's program model) and bridging them into a unified system. This repository teaches:

1. **Fundamentals** — What blocks, transactions, and confirmations actually are
2. **RPC communication** — How backends talk to blockchains
3. **Transaction lifecycle** — From creation to finality
4. **Multi-chain abstraction** — Building adapters for Bitcoin, EVM, Solana
5. **Exchange infrastructure** — Fund sweeping, custody, reconciliation, proof-of-reserves
6. **Production hardening** — Distributed systems, failover, security

Each lesson follows a disciplined format: **learning objective → blockchain concept → normal/special/exceptional cases → Rust design → comments-first implementation → tests → Rust vs Go comparison → interview preparation**.

## Proof Map

| Claim | Evidence |
|-------|----------|
| Multiple blockchain models require distinct abstractions | 01-blockchain-fundamentals, 07-bitcoin, 08-solana |
| RPC communication must isolate transport from domain logic | 02-evm-rpc, 03-evm-client |
| Transaction finality is more complex than "confirmed" | 04-transaction-lifecycle, 05-blockchain-indexer |
| Multi-chain design requires careful trait boundaries | 09-blockchain-gateway |
| Production systems need reconciliation + proof-of-reserves | 13-reconciliation, 14-proof-of-reserves |

## Structure

```
learn_blockchain_backend/
├── README.md              (this file)
├── PLAN.md                (full curriculum with progression)
├── CHEATSHEET.md          (quick blockchain reference)
│
├── 01-blockchain-fundamentals/    ← Start here
├── 02-evm-rpc/
├── 03-evm-client/
├── 04-transaction-lifecycle/
├── 05-blockchain-indexer/
├── 06-wallet/
├── 07-bitcoin/
├── 08-solana/
├── 09-blockchain-gateway/
├── 10-fund-sweeping/
├── 11-custody/
├── 12-hsm-mpc/
├── 13-reconciliation/
├── 14-proof-of-reserves/
└── 15-production-architecture/
```

Each section contains:

- `src/` — Implementation files (one concept per file)
- `tests/` — Unit and integration tests
- `examples/` — Runnable demonstrations
- `README.md` — Section objectives and progression
- `CONTEXT.md` — Blockchain concept deep-dive for this section

## Sections

1. **01-blockchain-fundamentals** — Blocks, transactions, confirmations, finality
2. **02-evm-rpc** — JSON-RPC protocol, communicating with EVM nodes
3. **03-evm-client** — Building a high-level EVM client abstraction
4. **04-transaction-lifecycle** — Transaction states, receipts, execution status
5. **05-blockchain-indexer** — Checkpointing, reorg handling, event parsing
6. **06-wallet** — Key management, address derivation, balance queries
7. **07-bitcoin** — UTXO model, inputs/outputs, transaction construction
8. **08-solana** — Account/program/instruction model, SPL tokens
9. **09-blockchain-gateway** — Unified abstraction across all three chains
10. **10-fund-sweeping** — Deposit consolidation, gas management, idempotency
11. **11-custody** — Hot/cold wallets, approval flows, risk checks
12. **12-hsm-mpc** — Hardware security modules, multi-party computation
13. **13-reconciliation** — Blockchain ↔ internal ledger verification
14. **14-proof-of-reserves** — Merkle trees, cryptographic proofs of solvency
15. **15-production-architecture** — HA, failover, monitoring, disaster recovery

## Running Tests

```bash
# Test all sections
cargo test

# Test a specific section
cargo test --package 01-blockchain-fundamentals

# Test with output
cargo test -- --nocapture
```

## File Structure for Each Lesson

Every implementation file follows this pattern:

```rust
// FILE: src/xxx.rs
//
// LEARNING OBJECTIVE
// Learn what X is and why backends need it.
//
// BLOCKCHAIN CONCEPT
// Explain the underlying concept (e.g., "A block is an immutable set of transactions").
//
// NORMAL CASE
// When everything works, what happens?
//
// SPECIAL CASES
// Valid but non-obvious scenarios.
//
// EXCEPTIONAL CASES
// Failures, malformed data, network issues.
//
// DESIGN DECISION
// Why we chose this Rust design over alternatives.
//
// --- IMPLEMENTATION FOLLOWS ---
```

Tests target **public behavior**, not implementation details. Rust vs Go comparisons highlight differences relevant to blockchain backend engineers.

## Tech Stack

- **Rust**: 1.70+ (standard library only for core lessons, then tokio/reqwest for networking)
- **Testing**: `#[cfg(test)]` built-in, `assert_eq!`, manual fixtures
- **Serialization**: `serde_json` (minimal; we often parse JSON manually to understand format)
- **HTTP**: `reqwest` (only in 02-evm-rpc onward)
- **Hashing**: `sha2` (for proof-of-reserves section)

No heavy frameworks; each lesson is self-contained and runnable with `cargo test`.

## Requirements

- Rust 1.70 or later: [https://rustup.rs](https://rustup.rs)
- Familiarity with blockchain basics (blocks, transactions) — but we teach the details
- Payment/exchange backend context helpful (but not required)

## How to Use This Repository

1. **Start with 01-blockchain-fundamentals**. Read the README in that section.
2. **Follow the comments-first pattern**. Code is documented *before* it runs.
3. **Run tests after each file**. Verify understanding with `cargo test --package 01-...`.
4. **Compare with Go**. Each Rust implementation includes an equivalent Go snippet.
5. **Do the exercises**. Some files include intentional gaps for you to fill.
6. **Reference the CHEATSHEET** when you forget blockchain concepts.
7. **Use PLAN.md** to understand the big picture and why this ordering matters.

## Interview Preparation

After completing this repository, you can answer:

- "Explain blockchain finality."
- "How does a backend distinguish transaction inclusion from execution?"
- "What's the difference between Bitcoin's and EVM's transaction model?"
- "How would you design a system that sweeps funds across chains?"
- "What happens if your indexer crashes mid-block?"
- "How do you prove reserves without exposing customer data?"
- "Why is distributed locking hard in blockchain systems?"

## Git Commit Convention

Each completed lesson gets one commit:

```
feat(01-blockchain-fundamentals): implement Block and Transaction types
```

## Contributing

This is a learning repository. Contributions should:
- Add or improve lessons following the comments-first pattern
- Include Rust implementation, tests, and Rust vs Go comparison
- Update PLAN.md and CHEATSHEET.md if introducing new concepts
- Keep lessons focused (one concept per file when possible)

---

**Next step**: Open `PLAN.md` to understand the full progression, then start [01-blockchain-fundamentals/README.md](01-blockchain-fundamentals/README.md).
