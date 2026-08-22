# Real-Chain Integration Plan

## Purpose

This document defines a separate integration track for connecting the blockchain backend concepts in this repo to real blockchain networks, instead of only local lesson models.

This is intentionally a separate path from the learning curriculum. The lesson repo remains the reference for concepts; the integration plan builds a real network layer on top of those ideas.

## Goal

Build a small but real blockchain integration service that can:

- connect to live or testnet blockchain nodes
- read balances and chain state
- submit signed transactions
- watch for confirmations and finality
- handle basic retries, errors, and idempotency
- support at least one EVM chain and one non-EVM chain

## Non-goals

- Do not turn this into a full production exchange backend yet
- Do not replace the conceptual lesson modules in this repo
- Do not assume a single wallet model for all chains
- Do not depend on a real mainnet wallet or a live customer fund flow in the first version

---

## Architecture

### 1. Network adapters

Create an adapter layer that isolates chain-specific behavior:

- `EvmRpcAdapter`
- `BitcoinRpcAdapter`
- `SolanaRpcAdapter`

Each adapter should expose common operations such as:

- health check
- get latest block
- get balance
- get transaction by hash
- broadcast signed transaction
- track confirmations

### 2. Shared service layer

Build a common integration service that uses adapters behind a trait interface:

- `BlockchainClient` trait
- `WalletSigner` trait
- `TransactionMonitor` trait
- `SubmissionResult` / `TxStatus` models

### 3. Configuration and environment

Use environment variables for chain parameters:

```env
RPC_URL_ETH=https://sepolia.infura.io/v3/...
RPC_URL_BTC=http://127.0.0.1:8332
RPC_URL_SOL=https://api.devnet.solana.com
PRIVATE_KEY_HEX=...
CHAIN=ethereum
NETWORK=sepolia
```

### 4. Operational boundaries

Keep these concerns separate:

- chain communication
- wallet signing
- transaction tracking
- retry / backoff
- error mapping
- persistence / checkpoints

---

## Recommended project structure

```text
real_chain_integration/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── traits.rs
│   ├── models.rs
│   ├── evm/
│   │   ├── adapter.rs
│   │   ├── tx.rs
│   │   └── client.rs
│   ├── bitcoin/
│   │   ├── adapter.rs
│   │   └── tx.rs
│   ├── solana/
│   │   ├── adapter.rs
│   │   └── tx.rs
│   ├── signer.rs
│   ├── monitor.rs
│   └── error.rs
├── .env.example
└── README.md
```

---

## Phase plan

### Phase 1 — Connection and read-only plumbing

Goal: prove the app can talk to a real chain.

Tasks:

1. Select one chain for the first pass: EVM testnet is the easiest entry point.
2. Add a JSON-RPC client using a real endpoint.
3. Implement health checks for node reachability.
4. Fetch latest block and account balance.
5. Log response payloads and normalize error types.

Acceptance criteria:

- node endpoint responds successfully
- latest block can be read
- account balance can be fetched
- error handling distinguishes RPC failures vs local logic failures

### Phase 2 — Signing and submission

Goal: submit a signed transaction to a real chain.

Tasks:

1. Create a wallet abstraction for the chosen chain.
2. Use a funded test account only.
3. Build unsigned transaction payload.
4. Serialize, sign, and broadcast.
5. Capture response and tx hash.

Acceptance criteria:

- signed transaction is accepted by the chain
- tx hash is returned
- failure modes are logged and handled

### Phase 3 — Monitoring and confirmations

Goal: observe status after broadcast.

Tasks:

1. Poll tx receipt or transaction status.
2. Track pending, mined, confirmed states.
3. Retry on transient RPC issues.
4. Record confirmations and finality information.

Acceptance criteria:

- tx status transitions are visible
- confirmations are counted correctly
- retries do not duplicate logic

### Phase 4 — Multi-chain adapter support

Goal: support multiple networks behind the same interface.

Tasks:

1. Add Bitcoin or Solana support after EVM is stable.
2. Reuse the same transaction workflow interface.
3. Add chain-specific serialization for addresses and payloads.
4. Keep adapters isolated from core logic.

Acceptance criteria:

- same service API works across supported chains
- adapter-specific code is confined to chain folders
- no chain logic leaks into the common service layer

### Phase 5 — Safety and hardening

Goal: move beyond demo code.

Tasks:

1. Add idempotency keys for submissions
2. Add retry budgets and backoff
3. Add structured logging and metrics
4. Add a local checkpoint store for pending tx tracking
5. Add test fixtures for failed RPC responses and reorg scenarios

Acceptance criteria:

- failed submissions are retried safely
- duplicate submissions are prevented or clearly handled
- operational state is recoverable

---

## Recommended first chain

Use Ethereum Sepolia (or a similar EVM testnet) first.

Why:

- easiest to debug with standard JSON-RPC
- broad tooling support
- good learning bridge from the repo's EVM sections
- straightforward balance and tx status monitoring

After the EVM flow is working, add Bitcoin regtest or Solana devnet as a second integration target.

---

## Security rules

- Never commit private keys to the repo
- Use `.env` or local secret storage only
- Use testnet or regtest funds only
- Keep all signing keys outside the repo in local secret management
- Never log raw private keys or seed phrases

---

## Validation checklist

Before considering the integration ready:

- [ ] can connect to a real node
- [ ] can fetch latest block
- [ ] can query balance
- [ ] can sign a transaction
- [ ] can broadcast a tx to a testnet
- [ ] can monitor status and confirmations
- [ ] can recover from transient RPC failures
- [ ] can run without leaking secrets

---

## Suggested milestone sequence

### Milestone 1

- read-only EVM integration works

### Milestone 2

- send a signed test transaction to Sepolia

### Milestone 3

- monitor transaction receipt and confirmation count

### Milestone 4

- add Bitcoin or Solana integration path

### Milestone 5

- add retry, observability, and checkpoint persistence

---

## Implementation guidance

Keep this work separate from the current lesson repo. The learning modules stay as they are; the integration project should be built as an adjacent service that consumes the same blockchain concepts.

The most important design decision is this:

- concept and lesson code should remain local-first and pedagogical
- real integration code should remain network-aware, operational, and environment-driven

---

## Next step

Start with this sequence:

1. create a new Rust crate under the repo root or a sibling folder
2. build a working EVM testnet adapter
3. integrate one funded test wallet
4. fetch block + balance
5. sign and send a low-value test transaction
6. monitor receipt and confirmations

This is the minimum viable path to a real-chain integration without mixing it into the teaching repo structure.
