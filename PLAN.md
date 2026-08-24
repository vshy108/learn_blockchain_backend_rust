# learn_blockchain_backend — PLAN

## Goal

Build educational models of blockchain gateway components by learning **one file at a time**, from fundamentals through production architecture. Each lesson teaches a single blockchain concept and produces working Rust code that illustrates part of a larger system.

## The Progression Logic

```
Understand blockchain
       ↓
Communicate with blockchain (RPC)
       ↓
Understand transactions
       ↓
Index blockchain
       ↓
Manage wallets
       ↓
Abstract multiple chains
       ↓
Build exchange infrastructure
       ↓
Add security/custody
       ↓
Production hardening
```

---

## Section 01: Blockchain Fundamentals

**Goal**: Understand what the backend is actually communicating with.

### Concepts
- Block, block height, block hash
- Transactions, inputs, outputs
- Confirmations vs finality
- Chain reorganization
- Account model (EVM) vs UTXO model (Bitcoin)

### Files (one at a time)
1. `01_block.rs` — Block structure, height, hash, timestamp
2. `02_transaction.rs` — Transactions, inclusion, execution
3. `03_account.rs` — Account model (EVM-style)
4. `04_address.rs` — Address encoding and validation
5. `05_confirmation.rs` — Confirmations, blocks-since-inclusion
6. `06_finality.rs` — Finalized vs non-finalized blocks

### Acceptance Criteria
- [ ] All tests pass: `cargo test --package blockchain_fundamentals`
- [ ] Can explain why "confirmed" ≠ "finalized"
- [ ] Can describe what a chain reorg is and why indexers care

---

## Section 02: EVM RPC Protocol

**Goal**: Learn how to communicate with blockchain nodes using JSON-RPC.

### Concepts
- JSON-RPC protocol structure (method, params, result, id)
- Hexadecimal encoding (0x...)
- RPC errors vs transport errors
- Request/response cycle

### Files
1. `01_rpc_request.rs` — Building JSON-RPC requests
2. `02_rpc_response.rs` — Parsing JSON-RPC responses
3. `03_hex_encoding.rs` — Converting between hex and integers
4. `04_block_number.rs` — eth_blockNumber RPC call
5. `05_get_block.rs` — eth_getBlockByNumber RPC call
6. `06_get_code.rs` — eth_getCode RPC call

### Acceptance Criteria
- [ ] Can construct valid JSON-RPC requests
- [ ] Can parse JSON-RPC responses and errors
- [ ] Understand transport errors vs RPC errors

---

## Section 03: EVM Client Layer

**Goal**: Build a high-level EVM client that abstracts RPC details.

### Concepts
- Transport layer separation
- Error handling patterns
- Retry logic
- Type safety (hex strings → u64, u256)

### Files
1. `01_evm_client.rs` — Basic client struct
2. `02_latest_block.rs` — Parse a latest-block result locally; async transport is a later extension
3. `03_get_block_details.rs` — Retrieve full block information
4. `04_account_state.rs` — Balance, nonce, code queries
5. `05_error_handling.rs` — Structured error types

### Acceptance Criteria
- [ ] Client abstracts RPC format from callers
- [ ] When live RPC transport is introduced, use Tokio for async calls
- [ ] Error types distinguish node errors from local errors

---

## Section 04: Transaction Lifecycle

**Goal**: Understand transaction states from creation to finality.

### Concepts
- Transaction states: created → signed → broadcast → pending → mined → confirmed
- Receipt, status, gas usage
- Failed transactions still cost gas
- Replacement transactions (nonce conflicts)

### Files
1. `01_transaction_struct.rs` — Transaction data structure
2. `02_signed_transaction.rs` — Signing and serialization
3. `03_transaction_receipt.rs` — Receipt parsing
4. `04_execution_status.rs` — Success vs revert
5. `05_confirmations.rs` — How many blocks since inclusion?
6. `06_transaction_state.rs` — Enum: Pending, Mined, Confirmed, Failed

### Acceptance Criteria
- [ ] Can distinguish transaction inclusion from execution success
- [ ] Receipt parsing handles all EVM fields
- [ ] State machine prevents invalid transitions

---

## Section 05: Blockchain Indexer

**Goal**: Learn how to robustly process blockchain state and handle crashes.

### Concepts
- Checkpointing (remembering "I processed up to block N")
- Crash recovery
- Chain reorganization handling
- Event parsing

### Files
1. `01_checkpoint.rs` — Save and restore progress
2. `02_block_range.rs` — Processing block ranges
3. `03_reorg_detection.rs` — Detecting when blocks change
4. `04_reorg_rollback.rs` — Undoing indexed state
5. `05_event_log.rs` — Parsing EVM logs/events
6. `06_indexer_state.rs` — Full indexer coordination

### Acceptance Criteria
- [ ] Can recover from checkpoint after crash
- [ ] Detects chain reorg and rolls back correctly
- [ ] Handles both initial sync and live indexing

---

## Section 06: Wallet Management

**Goal**: Manage cryptographic keys and derive addresses.

### Concepts
- Private key, public key, address derivation
- Key serialization and storage
- Signing transactions
- Test key best practices

### Files
1. `01_private_key.rs` — Private key storage and generation
2. `02_public_key.rs` — Public key derivation
3. `03_address.rs` — Address derivation and checksums
4. `04_signing.rs` — Signing with a private key
5. `05_verify_signature.rs` — Verifying signatures
6. `06_wallet.rs` — Full wallet abstraction

### Acceptance Criteria
- [ ] Private keys never logged or printed
- [ ] Address checksums validated
- [ ] All crypto uses standard Rust libraries (sha3, secp256k1)

---

## Section 07: Bitcoin (UTXO Model)

**Goal**: Learn Bitcoin's fundamentally different transaction model.

### Concepts
- UTXO (unspent transaction output)
- Inputs, outputs, change
- Mining fees
- Script (simplified)

### Files
1. `01_utxo.rs` — UTXO structure
2. `02_input_output.rs` — Transaction inputs/outputs
3. `03_change.rs` — Computing change
4. `04_fee_calculation.rs` — Fee estimation
5. `05_tx_construction.rs` — Building a transaction
6. `06_tx_broadcast.rs` — Sending to the network

### Acceptance Criteria
- [ ] Can explain why "balance" is derived, not stored
- [ ] Can construct a transaction with multiple inputs/outputs
- [ ] Understand fee mechanics and why change matters

---

## Section 08: Solana (Program Model)

**Goal**: Learn Solana's distinct account + program architecture.

### Concepts
- Accounts and program-derived addresses
- Instructions and transactions
- Lamports (SOL's base unit)
- SPL tokens (Solana's token standard)
- Commitment levels

### Files
1. `01_account.rs` — Solana account model
2. `02_program.rs` — Program-derived addresses
3. `03_instruction.rs` — Instructions and data
4. `04_transaction.rs` — Solana transaction construction
5. `05_spl_token.rs` — Token mints and accounts
6. `06_commitment.rs` — Processed, confirmed, finalized

### Acceptance Criteria
- [ ] Understand PDA (program-derived address) concept
- [ ] Can construct a Solana transaction
- [ ] Know why Solana's parallelism differs from EVM

---

## Section 09: Blockchain Gateway

**Goal**: Build a unified abstraction that works across Bitcoin, EVM, and Solana.

### Concepts
- Trait-based abstraction
- Adapter pattern
- When to NOT unify interfaces
- Chain-specific differences that matter

### Files
1. `01_blockchain_trait.rs` — Common interface definition
2. `02_bitcoin_adapter.rs` — Bitcoin → trait
3. `03_evm_adapter.rs` — EVM → trait
4. `04_solana_adapter.rs` — Solana → trait
5. `05_routing.rs` — Selecting the right adapter
6. `06_gateway.rs` — Full gateway implementation

### Acceptance Criteria
- [ ] Trait captures truly common operations
- [ ] Each adapter keeps chain-specific details
- [ ] No forced abstractions that hide important information

---

## Section 10: Fund Sweeping

**Goal**: Build exchange-style deposit consolidation.

### Concepts
- Deposit detection
- Sweep threshold
- Gas funding (native token needed to move other tokens)
- Idempotency
- Duplicate prevention

### Files
1. `01_sweep_detector.rs` — Finding sweepable deposits
2. `02_sweep_threshold.rs` — When to sweep
3. `03_gas_funding.rs` — Ensuring enough native token for gas
4. `04_sweep_transaction.rs` — Creating the sweep tx
5. `05_idempotency.rs` — Ensuring no duplicates
6. `06_sweep_tracker.rs` — Monitoring sweep status

### Acceptance Criteria
- [ ] Sweep only happens when threshold met
- [ ] Handles case where token exists but gas token doesn't
- [ ] No duplicate sweeps on restart

---

## Section 11: Custody

**Goal**: Implement secure key management and approval flows.

### Concepts
- Hot, warm, cold wallets
- Withdrawal request flow
- Risk policy checks
- Approval and signing separation
- Auditability

### Files
1. `01_custody_model.rs` — Wallet tiers
2. `02_withdrawal_request.rs` — Request structure
3. `03_risk_policy.rs` — Policy enforcement
4. `04_approval_flow.rs` — Multi-step approval
5. `05_signing_service.rs` — Separated signer
6. `06_audit_log.rs` — Tracking who approved what

### Acceptance Criteria
- [ ] Private keys in signing service only
- [ ] Risk checks applied before signing
- [ ] Full audit trail of approvals

---

## Section 12: HSM / MPC

**Goal**: Integrate hardware security modules or multi-party computation.

### Concepts
- HSM (hardware security module)
- MPC (multi-party computation)
- Quorum signing
- Key recovery

### Files
1. `01_hsm_interface.rs` — Abstract HSM interface
2. `02_hsm_integration.rs` — Real HSM calls (simulated)
3. `03_mpc_signer.rs` — MPC coordination
4. `04_key_recovery.rs` — Handling signer unavailability
5. `05_quorum.rs` — Threshold signatures
6. `06_rotation.rs` — Key rotation procedures

### Acceptance Criteria
- [ ] Application never sees private keys
- [ ] HSM unavailability is handled gracefully
- [ ] Can explain security model to auditor

---

## Section 13: Reconciliation

**Goal**: Verify blockchain state matches internal ledger.

### Concepts
- Blockchain source of truth
- Internal ledger state
- Discrepancy detection and resolution
- Auditing

### Files
1. `01_reconciliation_record.rs` — What we're reconciling
2. `02_blockchain_state.rs` — Querying blockchain
3. `03_ledger_state.rs` — Reading internal state
4. `04_discrepancy.rs` — Detecting differences
5. `05_resolution.rs` — Fixing discrepancies
6. `06_reconciliation_report.rs` — Audit trail

### Acceptance Criteria
- [ ] Can detect all classes of discrepancies
- [ ] Handles indexing lag gracefully
- [ ] Audit log explains every correction

---

## Section 14: Proof of Reserves

**Goal**: Prove exchange solvency without exposing customer data.

### Concepts
- Merkle trees
- Merkle proofs
- Commitment schemes
- Privacy-preserving verification

### Files
1. `01_merkle_tree.rs` — Building a merkle tree
2. `02_merkle_proof.rs` — Generating a proof
3. `03_merkle_verify.rs` — Verifying inclusion
4. `04_commitment.rs` — Committing to user balances
5. `05_liability_tree.rs` — Customer liability tree
6. `06_proof_report.rs` — Full proof-of-reserves report

### Acceptance Criteria
- [ ] Can prove one customer's inclusion without exposing others
- [ ] Root hash can be published and audited
- [ ] Math is sound (or clearly marked for review)

---

## Section 15: Production Architecture

**Goal**: Explore the design of a highly available blockchain gateway through production-oriented educational models.

### Concepts
- High availability
- RPC failover
- Distributed locking
- Nonce management
- Circuit breakers
- Observability

### Files
1. `01_ha_design.rs` — Architecture decisions
2. `02_rpc_pool.rs` — Multiple RPC endpoints
3. `03_failover.rs` — Switching on node failure
4. `04_nonce_manager.rs` — Safe nonce allocation
5. `05_circuit_breaker.rs` — Avoiding cascading failures
6. `06_monitoring.rs` — Observability instrumentation

### Acceptance Criteria
- [ ] System survives single RPC node failure
- [ ] Nonce conflicts prevented with distributed lock
- [ ] Monitoring surfaces key metrics
- [ ] Can onboard new chain in <1 day

---

## The Lesson Format (for every file)

Every implementation file follows this structure:

```
1. Learning objective
2. Blockchain concept
3. Normal case
4. Special cases ⚠️
5. Exceptional cases 🚨
6. Design decision
7. Comments first (documented before code)
8. Code (line by line)
9. Tests (public behavior, not implementation)
10. Verification (cargo test)
11. Rust vs Go comparison
12. Interview question
```

---

## Success Criteria

By the end, you should be able to:

- [ ] Explain blockchain finality to a senior engineer
- [ ] Distinguish transaction inclusion from execution
- [ ] Design a multi-chain abstraction without overfit
- [ ] Handle chain reorg in an indexer
- [ ] Build a sweep system with proper idempotency
- [ ] Explain custody models and risk policies
- [ ] Implement reconciliation and proof-of-reserves
- [ ] Design production HA architecture
- [ ] Answer any blockchain backend interview question

---

## Time Estimate

- Section 01 (fundamentals): 3-4 hours
- Sections 02-03 (RPC): 4-5 hours
- Sections 04-05 (lifecycle + indexing): 5-6 hours
- Sections 06-08 (wallets + other chains): 6-8 hours
- Sections 09-12 (gateway + custody): 8-10 hours
- Sections 13-15 (reconciliation + production): 8-10 hours

**Total**: 34-43 hours spread over 4-6 weeks (1-2 hours/day deliberate study).

---

## How to Use This Plan

1. **Start at Section 01**. Don't skip ahead.
2. **Complete one file at a time**. Write the comments first, then code.
3. **Run tests after each file**. Verify understanding before moving on.
4. **Reference CHEATSHEET.md** when you forget blockchain concepts.
5. **Compare with Go**. Each Rust implementation has a Go equivalent.
6. **Do the interview questions**. Practice articulating your knowledge.
7. **Build the gateway incrementally**. Each section adds functionality.

By Section 15, you will have built educational models of blockchain gateway components and learned the mental models required for production blockchain backend systems. The examples are not production-ready infrastructure.
