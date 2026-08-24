# Section 13: Reconciliation

## Goal

Verify blockchain state matches internal ledger. Detect and fix discrepancies.

> **Status:** Production-oriented learning model; it is not an automated reconciliation service for real assets.

## Why This Section

Here's a hard truth: your blockchain indexer, your database, and the actual blockchain will sometimes disagree. This section teaches how to detect and fix those discrepancies.

## The Problem

```
Blockchain says: 100 ETH in wallet 0xAAA
Internal ledger says: 99 ETH

Who's right? How did this happen? What do we do?
```

## Possible Causes

1. Indexer crashed, missed a transaction
2. Node had a reorg, indexer didn't detect it
3. Database bug (race condition, lost update)
4. Manual transaction (withdrawal, deposit) not yet indexed
5. Unexpected transfer (hack, test, accident)
6. Fee accounting error
7. Gas refund not credited

## Concepts You'll Learn

| Concept | Why It Matters |
|---------|---|
| Source of truth | Blockchain is the authority |
| Discrepancy types | Systematic vs one-off issues |
| Detection | Automated reconciliation |
| Resolution | Fixing state |
| Audit trail | Why every correction happened |

## Files You'll Create

1. `01_reconciliation_record.rs` — What we're reconciling
2. `02_blockchain_state.rs` — Querying blockchain
3. `03_ledger_state.rs` — Reading internal state
4. `04_discrepancy.rs` — Detecting differences
5. `05_resolution.rs` — Fixing discrepancies
6. `06_reconciliation_report.rs` — Audit trail

## The Flow

```
1. Query blockchain:  100 ETH at 0xAAA
2. Query ledger:      99 ETH at 0xAAA
3. Difference:        1 ETH missing
4. Investigate:
   - Re-process blocks 1000-2000
   - Find missing withdrawal
5. Correct ledger:    100 ETH
6. Log correction:    "Reprocessing found missed withdrawal TXN_123"
```

## Running Tests

```bash
cargo test --package reconciliation
```

## Acceptance Criteria

- [ ] Detect all classes of discrepancies
- [ ] Handle indexing lag gracefully
- [ ] Can re-process specific block ranges
- [ ] Audit log explains every correction
- [ ] Reconciliation is automated
- [ ] Ready for Section 14 (Proof-of-Reserves)

## Learning Check

- **Rust concepts:** structs, enums, comparisons, error classification, and audit-oriented data
- **Production problem:** detecting differences between an external source of truth and internal ledger state
- **Simplifications:** blockchain queries, ledger storage, concurrency, correction authorization, and persistence are modeled locally
- **Exercise:** add a test proving that an unresolved discrepancy cannot be marked as corrected

## Interview Questions

- "Your reconciliation finds a 1 ETH discrepancy. How do you investigate?"
- "You detect the same discrepancy for 100 customers. How is that handled differently?"
- "Daily reconciliation runs at 2 AM. At 10 AM, a discrepancy is discovered. Is it a bug?"

---

**Next**: Implement `01_reconciliation_record.rs`.
