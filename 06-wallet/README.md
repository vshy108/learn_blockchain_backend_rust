# Section 06: Wallet Management

## Goal

Learn cryptographic key management, address derivation, and transaction signing.

> **Status:** Learning model. The examples demonstrate concepts and are not a secure wallet implementation.

## Why This Section

Before you can build a blockchain backend, you need to understand keys and signatures. This section teaches key generation, address derivation, and the critical rule: **never log private keys**.

## Concepts You'll Learn

| Concept | Why It Matters |
|---------|---|
| Private key | Secret credential, 256 bits |
| Public key | Derived from private key |
| Address | Derived from public key |
| Signing | Creating signatures with private key |
| Verification | Proving signature without key |

## The Chain

```
Private Key (256-bit random)
     ↓ (secp256k1 multiplication)
Public Key (65 bytes, compressed to 33)
     ↓ (Keccak-256)
Address (20 bytes for EVM)
```

## Files You'll Create

1. `01_private_key.rs` — Key generation and storage
2. `02_public_key.rs` — Public key derivation
3. `03_address.rs` — Address derivation and checksums
4. `04_signing.rs` — Signing transactions
5. `05_verify_signature.rs` — Verifying signatures
6. `06_wallet.rs` — Full wallet abstraction

## Key Security Rules

These rules describe production concerns; the current examples use deterministic test data and simplified cryptographic behavior.

1. **Never print private keys** (even in tests with `dbg!()`)
2. **Never serialize private keys** to JSON/logs
3. **Never compare private keys** with `==` (timing attack risk)
4. **Use constant-time comparisons** for secrets
5. **Test with fake keys only** (never real private keys)

## Running Tests

```bash
cargo test --package wallet
```

## Acceptance Criteria

- [ ] Private keys never logged or serialized
- [ ] Address checksums validated (EVM only)
- [ ] Signing and verification work correctly
- [ ] Wallet can derive addresses deterministically
- [ ] All crypto uses standard Rust libraries
- [ ] Tests use fake/test keys only
- [ ] Ready for Section 07 (Bitcoin)

## Learning Check

- **Rust concepts:** fixed-size arrays, `Option`, ownership, borrowing, and error-aware APIs
- **Production problem:** separating key material, public identity, signing, and verification responsibilities
- **Simplifications:** test keys and simplified cryptographic behavior are used; no secure key storage or audited signing implementation is provided
- **Exercise:** add a test proving that an invalid signature is rejected without exposing private-key data

## Interview Questions

- "Explain the secp256k1 curve and why it's used for blockchain."
- "How do you safely serialize a wallet to disk without exposing the private key?"
- "What's the difference between signing and verification, and why can verification work without the private key?"

---

**Next**: Implement `01_private_key.rs`.
