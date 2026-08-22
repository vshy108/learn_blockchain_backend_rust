# Section 07: Bitcoin (UTXO Model)

## Goal

Understand Bitcoin's fundamentally different transaction model: UTXOs instead of accounts.

## Why This Section

You've learned EVM (accounts). Bitcoin is completely different. This teaches you a second mental model, which is essential for building a multi-chain gateway. Solana will be the third.

## The Core Difference

### EVM (Account Model)
```
Address → Balance (always up to date)
```
Sending: balance -= amount

### Bitcoin (UTXO Model)
```
A UTXO is a previous output you can spend
(You must explicitly select which outputs to spend)
```
Sending: select inputs (outputs) → create new outputs → fee = input_sum - output_sum

## Concepts You'll Learn

| Concept | Why It Matters |
|---------|---|
| UTXO | "Coins" are actually outputs |
| Input | Spending a previous output |
| Output | Creating new coins |
| Change | Money back to yourself |
| Fee | Difference between input and output |

## Files You'll Create

1. `01_utxo.rs` — UTXO structure and validation
2. `02_input_output.rs` — Transaction inputs/outputs
3. `03_change.rs` — Change calculation
4. `04_fee_calculation.rs` — Fee estimation
5. `05_tx_construction.rs` — Building a transaction
6. `06_tx_broadcast.rs` — Sending to network

## Key Data Structures

### UTXO
```rust
pub struct UTXO {
    pub outpoint: Outpoint,  // which TX and index
    pub amount: u64,         // satoshis
    pub script_pubkey: Vec<u8>,  // who can spend it
}

pub struct Outpoint {
    pub tx_id: [u8; 32],
    pub index: u32,
}
```

### Transaction
```rust
pub struct Transaction {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

pub struct Input {
    pub outpoint: Outpoint,
    pub script_sig: Vec<u8>,  // proof of ownership
}

pub struct Output {
    pub amount: u64,
    pub script_pubkey: Vec<u8>,
}
```

## The Fundamental Pattern

```
1. Find UTXOs I can spend
   utxos = [UTXO(5 BTC), UTXO(3 BTC)]

2. Select inputs
   inputs = [UTXO(5 BTC), UTXO(3 BTC)]
   total_input = 8 BTC

3. Create outputs
   outputs = [
     Output(7 BTC to recipient),
     Output(0.9 BTC to my change)
   ]
   total_output = 7.9 BTC

4. Fee
   fee = 8 BTC - 7.9 BTC = 0.1 BTC

5. Sign and broadcast
```

## Running Tests

```bash
cargo test --package 07-bitcoin
```

## Acceptance Criteria

- [ ] Understand why "balance" is derived, not stored
- [ ] Can construct multi-input/multi-output transactions
- [ ] Change calculation is correct
- [ ] Fee calculation matches expectation
- [ ] Transaction serialization is valid
- [ ] Ready for Section 08 (Solana)

## Interview Questions

- "Why doesn't Bitcoin have an account balance field?"
- "You need to send 5 BTC but only have UTXOs of 3+3. How do you construct the transaction?"
- "What happens if you don't send change back to yourself?"

---

**Next**: Implement `01_utxo.rs`.
