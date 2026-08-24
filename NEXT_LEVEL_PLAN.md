# Next-Level Plan

## Purpose

This document describes future work after the 15-section educational curriculum in `PLAN.md`.
It focuses on turning local learning models into carefully verified, testnet-oriented exercises.
The repository remains educational and production-oriented; it is not production-ready infrastructure.

## Scope

Keep `PLAN.md` as the authoritative curriculum for Sections 01 through 15.
Use this document for the next implementation stage after the existing lessons are understood.
Work incrementally, one vertical slice at a time, with tests and explicit limitations.

## Phase 0: Baseline and Learning Evidence

### Goal

Establish a reproducible green baseline and record what the current models prove.

### Work

- Keep all 15 section READMEs aligned with their source files.
- Preserve educational status labels and limitation notes.
- Keep the CI quality gate active.
- Record Rust concepts, production context, assumptions, and exercises for each section.

### Acceptance Checks

- `cargo fmt --all -- --check` passes.
- `cargo clippy --workspace --all-targets -- -D warnings` passes.
- `cargo check --workspace` passes.
- `cargo test --workspace` passes.
- The examples are clearly labeled as educational or production-oriented models.

## Phase 1: Read-Only Real-Chain Integration

### Goal

Add a read-only EVM testnet client without exposing secrets or sending transactions.

### Work

- Add environment-driven RPC configuration.
- Implement latest-block and balance reads.
- Distinguish configuration, transport, RPC, and parsing errors.
- Add mock-server tests.
- Add an opt-in Sepolia smoke test that requires an external RPC URL.
- Document timeouts, rate limits, and secret-safe logging.

### Dependencies

Build on Sections 02 and 03. Use `REAL_CHAIN_INTEGRATION.md` as the integration reference.

### Acceptance Checks

- Mock tests pass without credentials or network access.
- Invalid configuration fails with a typed, understandable error.
- The opt-in testnet check retrieves a block and balance when configured.
- RPC URLs, tokens, and response secrets are not logged.
- New crates are justified against the standard library before being added.

## Phase 2: Testnet Transaction Lifecycle

### Goal

Extend the read-only client into a test-wallet-only transaction workflow.

### Work

- Add test-only key handling using audited cryptographic crates.
- Serialize and sign a transaction correctly for the selected testnet.
- Broadcast the transaction and poll its receipt.
- Track pending, mined, reverted, confirmed, and finalized states.
- Add bounded retries, timeouts, and idempotent submission tracking.

### Dependencies

Complete Phase 1 and the transaction lifecycle concepts in Section 04 first.

### Acceptance Checks

- Unit tests cover success, revert, timeout, retry, replacement, and duplicate submission cases.
- No mainnet keys or funds are used.
- An opt-in testnet transaction reaches a documented terminal state.
- Failed or timed-out polling does not loop forever.
- Private keys never appear in logs, errors, fixtures, or output.

## Phase 3: Second-Chain Adapter

### Goal

Add one second-chain integration while preserving chain-specific semantics.

### Work

- Choose Bitcoin regtest or Solana devnet.
- Keep chain-specific models, serialization, and errors inside the adapter.
- Keep shared gateway traits limited to genuinely common operations.
- Add adapter contract tests for supported shared behavior.

### Dependencies

Build on Section 07 or 08 and the gateway boundaries in Section 09.

### Acceptance Checks

- Tests run without credentials.
- Shared workflows work where the semantics genuinely overlap.
- Chain-specific behavior is not hidden behind misleading generic methods.
- Adapter errors identify the chain and failure category clearly.

## Phase 4: Recoverability and Operations

### Goal

Make failure, restart, reorganization, and observability behavior testable.

### Work

- Add persistent checkpoint storage.
- Add restart and crash-recovery fixtures.
- Add reorg detection, rollback, and idempotent event processing.
- Add RPC health checks, failover, and circuit breaking.
- Add structured events, metrics, and tracing boundaries.
- Document retry budgets, timeouts, and recovery assumptions.

### Dependencies

Build on Sections 05, 13, and 15.

### Acceptance Checks

- Injected node failures recover within a bounded retry budget.
- Restarts do not duplicate indexed events or sweeps.
- Reorg rollback and reprocessing produce the expected state.
- Health, latency, retry, and failure signals are observable.
- Persistence and distributed coordination limitations are documented.

## Phase 5: Educational Capstone and Security Review

### Goal

Connect the lessons into one reproducible local scenario and review its trust boundaries.

### Work

- Connect gateway, indexer, sweeping, custody, reconciliation, and proof-of-reserves models.
- Simulate a deposit, confirmation, ledger update, withdrawal approval, signing boundary, and audit record.
- Write a threat model covering keys, RPC trust, replay, reorgs, duplicate work, and operator access.
- Review cryptography, authorization, privacy, error handling, and observability assumptions.
- Document which parts require audited libraries, secure services, or independent review.

### Acceptance Checks

- The local scenario is reproducible from documented commands.
- Tests prove the important state and authorization boundaries.
- The threat model identifies assets, actors, trust assumptions, and mitigations.
- Every simulated component is labeled clearly.
- No example is presented as production-ready infrastructure.

## Explicit Exclusions

This roadmap does not include the following by default:

- Mainnet funds or mainnet transaction execution.
- Real customer flows or customer financial data.
- Claims that local code is production HSM or MPC security.
- Deployment infrastructure unrelated to the current learning objective.
- Custom cryptography when an audited library or provider is appropriate.

Any excluded area requires a separate design decision, threat model, dependency review, and acceptance plan.

## References

- `PLAN.md` - authoritative 15-section curriculum.
- `README.md` - repository purpose, structure, commands, and limitations.
- `REAL_CHAIN_INTEGRATION.md` - existing real-chain integration guidance.
- `CHEATSHEET.md` - blockchain terminology and chain distinctions.
- `ERRORS_FIXED.md` - recorded workspace fixes and validation evidence.
- `.github/workflows/rust.yml` - formatting, Clippy, compilation, and test quality gate.
- Section READMEs - concepts, status labels, acceptance criteria, and learning checks.

## Working Rules

- Start each phase with a small, behavior-scoped acceptance check.
- Prefer local deterministic tests before network-dependent checks.
- Explain design and tradeoffs before implementation.
- Verify every external API against the actual dependency versions.
- Keep generated code as a draft until it is simplified and tested.
- Run the narrowest relevant check after each change.
