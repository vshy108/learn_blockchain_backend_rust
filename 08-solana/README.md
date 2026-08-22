# Section 08: Solana (Program Model)

## Goal

Learn Solana's distinct model: accounts + programs + instructions. Your third mental model.

## Why This Section

Bitcoin = UTXO. EVM = accounts. Solana = accounts + programs + instructions. Understanding all three makes you a true multi-chain backend engineer. Solana's model is particularly important for building gateways because it's fundamentally parallel.

## The Core Model

```
Account = {
  pubkey,
  balance (lamports),
  data (arbitrary bytes),
  owner (which program can modify),
  executable (is this a program?)
}

Instruction = {
  program_id (which program to call),
  accounts (which accounts are involved),
  data (instruction opcode + args)
}

Transaction = {
  instructions: [ix1, ix2, ...],
  signatures: [signer1, signer2, ...],
}
```

All instructions in a TX **must succeed or the entire TX fails** (atomic).

## Concepts You'll Learn

| Concept | Why It Matters |
|---------|---|
| Account | State storage (not just balance) |
| PDA | Program-derived address (deterministic) |
| Instruction | Function call to a program |
| SPL Token | Solana's token standard (like ERC-20) |
| Commitment | Processed, confirmed, finalized |

## Files You'll Create

1. `01_account.rs` — Account structure
2. `02_program.rs` — Program-derived addresses (PDA)
3. `03_instruction.rs` — Instructions and data
4. `04_transaction.rs` — Transaction construction
5. `05_spl_token.rs` — Token mints and accounts
6. `06_commitment.rs` — Commitment levels

## Key Differences from EVM

| Aspect | EVM | Solana |
|--------|-----|--------|
| State | Account balance (1 field) | Account data (arbitrary bytes) |
| Parallelism | Sequential (global state) | Parallel (accounts independent) |
| Atomicity | Single contract call | Entire transaction atomic |
| Fees | Per-operation (gas) | Per-transaction (5000 lamports base) |

## Running Tests

```bash
cargo test --package 08-solana
```

## Acceptance Criteria

- [ ] Understand account model deeply
- [ ] Can derive PDAs deterministically
- [ ] Can construct instructions and transactions
- [ ] Know difference between SPL token mint and account
- [ ] Understand commitment levels and finality
- [ ] Ready for Section 09 (Gateway)

## Interview Questions

- "What is a program-derived address and why can't it have a private key?"
- "A Solana transaction contains 3 instructions. Instruction 1 succeeds, instruction 2 fails. What happens to instruction 3?"
- "Compare Solana's account model to Ethereum's. Why is Solana more parallelizable?"

---

**Next**: Implement `01_account.rs`.
