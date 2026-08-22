# Blockchain Cheatsheet

Quick reference for blockchain concepts. For deep dives, see CONTEXT.md in each section.

## Core Concepts

### Block
- **Block**: An immutable set of transactions, timestamp, and reference to previous block
- **Block Height** (number): Sequential count starting from 0 (genesis)
- **Block Hash**: SHA-256 of block header; universally unique
- **Previous Block Hash**: Cryptographic link to prior block (chain)

```
Block 100
├── Height: 100
├── Hash: 0xabcd...
├── Previous Hash: 0x1234... (Block 99)
├── Timestamp: 1692345678
└── Transactions: [tx1, tx2, tx3]
```

### Confirmation
- **Confirmation**: Number of blocks mined since transaction inclusion
- **Confirmation = 1**: Transaction in the most recent block
- **Confirmation = 6**: Transaction 6 blocks back (safe in most chains)
- **NOT finality**: Even confirmed blocks can disappear if chain reorganizes

### Finality
- **Finalized**: A block that **cannot** be changed, even by a chain reorg
- **Bitcoin**: ~6 confirmations = practical finality (51% attack requires 6 blocks)
- **EVM (PoS)**: Epoch + 2 = finality (~2 minutes on Ethereum)
- **Solana**: Depends on commitment level (processed, confirmed, finalized)

### Chain Reorganization
```
      102A ─── 103A ─── ...
     /
101 ─
     \
      102B ─── 103B ─── ...
```
- The "correct" chain is now 102B, 103B (not 102A, 103A)
- Indexer must detect and undo indexed state from 102A
- Transactions in 102A may reappear in 102B or disappear entirely

---

## Bitcoin (UTXO Model)

### UTXO
- **UTXO**: Unspent Transaction Output
- A "coin" is actually an **unspent output** from a prior transaction
- Spending = selecting inputs (old outputs) and creating new outputs

### Transaction
```
Input:  Previous UTXO (tx_id, index, scriptPubKey proof)
        Previous UTXO (tx_id, index, scriptPubKey proof)
        ...
        ↓
        (value consumed)
        ↓
Output: Recipient address + amount
        Change address + amount
        ...
        ↓
Fee:    Input sum - Output sum = fee
```

### Balance
- "Balance" is derived: sum of all unspent outputs you can spend
- No central balance field on chain
- Must query all UTXOs to compute balance

---

## Ethereum / EVM

### Account Model
- Each address has a **balance** (stored on-chain)
- Each address has a **nonce** (transaction counter for ordering)
- Balance + Nonce = state

```
0x742d35Cc6634C0532925a3b844Bc9e7595f42bE
├── Balance: 5.5 ETH
├── Nonce: 42
└── Code: (smart contract bytecode, if account is a contract)
```

### Transaction
```
{
  from: 0x742d35Cc...,
  to: 0x1234567...,
  value: 1 ETH,
  data: 0x... (calldata for contract interaction),
  gas: 21000,
  gasPrice: 50 gwei,
  nonce: 42
}
```

### Execution
- TX broadcast → TX in mempool (pending)
- Miner/validator includes in block → TX mined
- **Mined ≠ executed successfully**

```
TX receipt {
  status: 1 (success) or 0 (revert)
  gasUsed: 21000,
  logs: [events emitted]
}
```

### Gas
- **Gas**: Measure of computation cost
- **Gas Price**: ETH per unit of gas (market-driven)
- **Total Cost**: gas × gasPrice (+ any value sent)
- **Revert doesn't refund gas**: Even failed TXs cost gas

---

## Solana (Program Model)

### Account
```
{
  pubkey: 0x...,
  balance: lamports (SOL base unit),
  data: arbitrary bytes,
  owner: which program can modify this,
  executable: true (if a program)
}
```

### Program-Derived Address (PDA)
- Deterministic address derived from program ID + seed
- Signing key is the program itself (not traditional wallet)
- Used for program-controlled state storage

### Instruction
```
{
  program_id: which program to call,
  accounts: [list of account metas],
  data: instruction data (opcode + args)
}
```

### Transaction
```
{
  instructions: [ix1, ix2, ix3],
  signatures: [signer1, signer2, ...]
}
```
- All instructions **must** succeed or entire TX fails (atomic)
- Can call multiple programs in single TX

### Commitment Level
- **Processed**: Blockchain received it
- **Confirmed**: Majority of validators agree
- **Finalized**: Validators cannot revert (10-30 seconds)

---

## JSON-RPC Format

All blockchain RPC uses JSON-RPC 2.0:

```json
{
  "jsonrpc": "2.0",
  "method": "eth_blockNumber",
  "params": [],
  "id": 1
}
```

Response (success):
```json
{
  "jsonrpc": "2.0",
  "result": "0x15",
  "id": 1
}
```

Response (error):
```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32601,
    "message": "Method not found"
  },
  "id": 1
}
```

### Hexadecimal in EVM
- Numbers encoded as `0x...` (hex)
- `0x15` = 21 (decimal)
- `0xff` = 255 (decimal)
- Must parse hex strings to numbers

---

## Hash Functions

### SHA-256 (Bitcoin)
- Input: bytes → Output: 32 bytes (256 bits)
- Used for: block hashing, transaction IDs (txid)
- **Double SHA-256**: SHA-256(SHA-256(x)) for txid

### Keccak-256 (EVM)
- Input: bytes → Output: 32 bytes (256 bits)
- Used for: block hashing, event log topics, storage keys
- Different from SHA-3 (EVM came first)

### BLAKE3 (Solana)
- Input: bytes → Output: 32 bytes (256 bits)
- Newer hash, designed for speed and parallelism

---

## Signing & Verification

### ECDSA (Elliptic Curve Digital Signature Algorithm)
- **Private key**: 256-bit random number
- **Public key**: Point on secp256k1 curve (derived from private key)
- **Address**: Hash of public key (shortened)

```
Private Key (256 bits)
     ↓ (derive)
Public Key (65 bytes, compressed to 33)
     ↓ (hash)
Address (20 bytes for EVM, varies for Bitcoin/Solana)
```

### Sign & Verify
```
Sign(message, private_key) → signature (r, s)
Verify(message, signature, public_key) → bool
```

---

## Confirmation vs Finality Summary

| Chain | 1 Confirmation | Practical Finality |
|-------|---|---|
| Bitcoin | Next block | ~10 minutes (6 blocks) |
| Ethereum (PoS) | ~12 seconds | ~2 minutes (2 epochs) |
| Solana | ~400ms | ~30 seconds |

**Rule of thumb**: Finality > practical confirmation for user withdrawals.

---

## Reconciliation Checklist

When blockchain state ≠ internal ledger:

- [ ] Indexer crashed and has stale checkpoint
- [ ] Node had a reorg, didn't rollback state
- [ ] Internal ledger bug (math error, race condition)
- [ ] Manual transaction (withdrawal, deposit) not yet indexed
- [ ] Unexpected transfer (hack, test, accident)
- [ ] Fee accounting error
- [ ] Gas refund not credited

**Fix process**:
1. Identify which blocks need re-indexing
2. Rollback state to before issue
3. Re-process with fix
4. Audit log: what, why, by whom

---

## Proof of Reserves

**Goal**: Prove `∑(customer balances) ≤ blockchain assets` without exposing individual balances.

### Merkle Tree Approach
1. Leaf = hash(customer_id, balance)
2. Parent = hash(left_child, right_child)
3. Root = public commitment
4. Proof = path from leaf to root
5. Verifier can confirm one customer without seeing others

```
           Root
          /    \
      Node1    Node2
      /  \      /  \
   C1 C2 C3 C4
```
- Publish Root
- Customer C1 can verify with: C1, C2, Node2 (other branches omitted)

---

## Common RPC Calls

### EVM (Ethereum)

```
eth_blockNumber → current block height
eth_getBlockByNumber(number, includeTransactions) → block
eth_getTransactionByHash(hash) → transaction
eth_getTransactionReceipt(hash) → receipt (after mined)
eth_getBalance(address, blockNumber) → balance
eth_getCode(address, blockNumber) → contract bytecode
eth_call(tx, blockNumber) → result (read-only execution)
eth_sendTransaction(tx) → txhash (broadcast)
eth_getLogs(filter) → events
```

### Bitcoin

```
getblockcount → current height
getblock(hash) → block
gettransaction(txid) → transaction
getrawtransaction(txid) → raw bytes
sendrawtransaction(hex) → txid
```

### Solana

```
getSlot() → current slot
getBlock(slot) → block
getTransaction(signature) → transaction
getBalance(pubkey) → balance
```

---

## Error Types

### Transport Errors
- Network unreachable
- Node timeout
- HTTP 500

**Action**: Retry, failover to another node

### RPC Errors
- Invalid method
- Parameter format error
- "Block not found" (reorg happened)

**Action**: Validate request, handle block reorg, retry

### Blockchain Errors
- Insufficient balance
- Nonce too low (already used)
- Invalid signature

**Action**: Fix the transaction, don't retry blindly

---

## Production Readiness Checklist

- [ ] Never log private keys or raw signed transactions
- [ ] Nonce management with distributed lock
- [ ] RPC failover to 2+ nodes
- [ ] Retry logic with exponential backoff
- [ ] Circuit breaker for cascading failures
- [ ] Monitoring: transaction volume, confirmation latency, errors
- [ ] Audit log: every approval, every signature
- [ ] Recovery plan: how to rebuild indexer state
- [ ] Security: custody separation, risk policy, approval flow
- [ ] Reconciliation: blockchain state vs ledger, daily audit

---

**For detailed explanations, see each section's CONTEXT.md file.**
