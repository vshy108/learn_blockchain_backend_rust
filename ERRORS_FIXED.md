# Errors Fixed - learn_blockchain_backend_rust

This log reflects the actual issues that were fixed while building and stabilizing the Rust learning repo. The final workspace was verified with:

```bash
cd /Users/00156637hopenghou/Repo/learn_blockchain_backend_rust && cargo test --workspace -- --nocapture 2>&1 | tail -n 200
```

Result: exit code 0, with no failing tests in the final output.

## Error 1: Package names cannot start with digits
**Date**: 2026-08-23
**File**: section Cargo manifests
**Problem**:
```text
error: invalid character `0` in package name: `01-blockchain-fundamentals`
```
**Cause**: Cargo package names cannot begin with a digit.
**Fix**: normalized the package names to valid Rust identifiers using underscores, e.g. `blockchain_fundamentals`.
**Applied To**: section Cargo manifests and workspace root configuration

## Error 2: Duplicate workspace.package definitions
**Date**: 2026-08-23
**File**: Cargo.toml (root)
**Problem**:
```text
error: duplicate key
  --> Cargo.toml
```
**Cause**: duplicate `[workspace.package]` sections were introduced during repo scaffolding.
**Fix**: kept one canonical `[workspace.package]` block and removed the duplicate.
**Applied To**: root Cargo.toml

## Error 3: Missing manifests for workspace members
**Date**: 2026-08-23
**File**: section directories
**Problem**:
```text
failed to load manifest for workspace member ...
Caused by: failed to read .../Cargo.toml
```
**Cause**: the workspace listed member folders but their Cargo manifests had not been created.
**Fix**: created Cargo.toml for each section package.
**Applied To**: sections 02-15

## Error 4: Missing source targets for manifests
**Date**: 2026-08-23
**File**: section Cargo.toml files
**Problem**:
```text
error: no targets specified in the manifest
```
**Cause**: manifests existed but had no `src/lib.rs`, `src/main.rs`, or `[[bin]]` targets.
**Fix**: added the required source files and bin declarations, then kept the repo aligned with the six-file-per-section lesson structure.
**Applied To**: all later lesson packages

## Error 5: Wrong assertion in transaction lifecycle logic
**Date**: 2026-08-23
**File**: 04-transaction-lifecycle/src/01_transaction_struct.rs
**Problem**: a unit test failed on the expected total cost value.
**Cause**: the assertion was incorrect relative to the actual implementation and transaction math.
**Fix**: corrected the test expectation to the valid value `21_000_010`.
**Applied To**: transaction lifecycle example and verification run

## Error 6: Missing later-section source files and bin targets
**Date**: 2026-08-23
**Files**:
- 09-blockchain-gateway
- 11-custody
- 12-hsm-mpc
- 13-reconciliation
- 14-proof-of-reserves
- 15-production-architecture
**Problem**:
```text
error: can't find bin ... at path .../src/xx_file.rs
```
**Cause**: the package manifests referenced six files per section, but several files were missing.
**Fix**: created the missing files and aligned their content to the repo's lesson format.
**Applied To**: all remaining sections after the initial set

## Final status
- repo scaffold is stable
- workspace builds successfully
- lessons remain local-first and teachable
- real-chain integration is intentionally kept as a separate plan, not part of the learning curriculum

## Verified outcome
- `cargo test --workspace -- --nocapture` completed successfully
- final exit code: 0
- no failing tests remained in the final validation run
