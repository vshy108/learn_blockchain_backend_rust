# Section 11: Custody

## Goal

Implement secure key management, withdrawal approval flows, and risk policies.

## Why This Section

Holding customer funds is the core responsibility of an exchange. This section teaches:
- Separation of keys and signing
- Risk policy enforcement
- Approval workflows
- Auditability

## The Model

```
User → Withdrawal Request
         ↓
      Risk Policy Check
         ↓
      Approver Review
         ↓
      Approve/Reject
         ↓
         (if approved)
         ↓
      Signer (in HSM/secure location)
         ↓
      Broadcast TX
```

## Wallet Tiers

```
Hot Wallet:   On server, mostly automated
              (small daily limits)

Warm Wallet:  Semi-secure, requires manual approval
              (medium limits, 1-2 approvers)

Cold Wallet:  Offline/very secure, rare use
              (large limits, multi-sig, 3+ approvers)
```

## Concepts You'll Learn

| Concept | Why It Matters |
|---------|---|
| Custody model | Which wallet for which amounts |
| Risk policy | Limits per user, per day, per withdrawal |
| Approval flow | Multi-step authorization |
| Signing service | Separated from approval |
| Audit log | Proving who approved what |

## Files You'll Create

1. `01_custody_model.rs` — Wallet tiers
2. `02_withdrawal_request.rs` — Request structure
3. `03_risk_policy.rs` — Policy enforcement
4. `04_approval_flow.rs` — Multi-step approval
5. `05_signing_service.rs` — Separated signer
6. `06_audit_log.rs` — Full audit trail

## Running Tests

```bash
cargo test --package 11-custody
```

## Acceptance Criteria

- [ ] Private keys only in signing service
- [ ] Risk policies enforced before signing
- [ ] Approval flow prevents unauthorized withdrawals
- [ ] Full audit trail of all approvals
- [ ] Wallet tier logic correct
- [ ] Ready for Section 12 (HSM/MPC)

## Interview Questions

- "A customer requests to withdraw $100k USD worth of crypto. How do you determine which wallet to use?"
- "Your daily limit for a user is $50k. Two requests arrive for $30k each in different cities. What happens?"
- "Explain how you prevent a hacked admin account from stealing funds."

---

**Next**: Implement `01_custody_model.rs`.
