# Section 10: Fund Sweeping

## Goal

Build an exchange-style deposit consolidation system. Detect deposits and move them to a treasury wallet.

## Why This Section

This is the first "real-world problem" section. You'll learn how exchanges actually work: deposits arrive in many wallets, and you need to sweep them to a central treasury efficiently.

## The Problem

```
User deposits 1 ETH to 0xAAA...
User deposits 2 ETH to 0xBBB...
User deposits 0.5 ETH to 0xCCC...
                ↓
         Sweep to Treasury
                ↓
Treasury now holds: 3.5 ETH
```

## Special Cases

### Gas Funding
```
ERC-20 token in 0xAAA: 1000 USDC
ETH in 0xAAA: 0 (no gas!)

Can't transfer USDC without ETH for gas.
Solution: Send ETH first, then sweep USDC.
```

### Threshold
```
Only sweep if balance > threshold
(Don't waste gas fees on tiny amounts)
```

### Idempotency
```
If sweep TX fails:
  Retry
  If same TX sent again, don't duplicate
```

## Concepts You'll Learn

| Concept | Why It Matters |
|---------|---|
| Deposit detection | Finding sweepable wallets |
| Threshold | Cost-benefit of sweeping |
| Gas funding | Two-step transfers for tokens |
| Idempotency | No duplicate sweeps |
| Monitoring | Tracking sweep status |

## Files You'll Create

1. `01_sweep_detector.rs` — Finding deposits to sweep
2. `02_sweep_threshold.rs` — When to sweep
3. `03_gas_funding.rs` — Ensuring native token for gas
4. `04_sweep_transaction.rs` — Creating sweep TX
5. `05_idempotency.rs` — Preventing duplicates
6. `06_sweep_tracker.rs` — Status monitoring

## Running Tests

```bash
cargo test --package 10-fund-sweeping
```

## Acceptance Criteria

- [ ] Detect sweepable deposits correctly
- [ ] Sweep only when threshold met
- [ ] Handle token without gas case
- [ ] No duplicate sweeps on restart
- [ ] Track all sweep attempts
- [ ] Ready for Section 11 (Custody)

## Interview Questions

- "You have 100 deposits. How do you sweep them efficiently?"
- "A sweep TX fails because insufficient gas. How do you recover?"
- "What happens if you send the sweep TX twice by accident?"

---

**Next**: Implement `01_sweep_detector.rs`.
