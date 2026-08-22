# Section 12: HSM / MPC

## Goal

Learn hardware security modules and multi-party computation for distributed signing.

## Why This Section

The most dangerous thing in a blockchain backend is the private key. This section teaches architectures where **no single machine holds the key**.

## Two Approaches

### HSM (Hardware Security Module)
```
Application
     ↓
HSM (physical device, US$500-50k)
     ↓
"Sign this transaction"
     ↓
[HSM does the signing internally, never exposes key]
     ↓
Signature returned
```
Signature lives forever, key never leaves the device.

### MPC (Multi-Party Computation)
```
Party A (holds key_shard_A)
Party B (holds key_shard_B)
Party C (holds key_shard_C)
     ↓
[Signing ceremony]
     ↓
All three coordinate → signature
     ↓
No party ever has the full key
```

## Concepts You'll Learn

| Concept | Why It Matters |
|---------|---|
| HSM interface | Communicating with hardware |
| MPC coordination | Distributed signing |
| Quorum | Threshold (2 of 3, etc.) |
| Unavailability | Handling signer down |
| Key recovery | Emergency procedures |

## Files You'll Create

1. `01_hsm_interface.rs` — Abstract HSM trait
2. `02_hsm_integration.rs` — Real HSM simulation
3. `03_mpc_signer.rs` — MPC coordination
4. `04_key_recovery.rs` — Handling unavailability
5. `05_quorum.rs` — Threshold signatures
6. `06_rotation.rs` — Key rotation

## Important Note

We do **not** implement production MPC cryptography ourselves. We learn the architecture and how a backend integrates with it.

## Running Tests

```bash
cargo test --package 12-hsm-mpc
```

## Acceptance Criteria

- [ ] Understand HSM architecture and limitations
- [ ] MPC signing coordination works
- [ ] Application never sees private key
- [ ] Graceful handling of signer unavailability
- [ ] Rotation procedures documented
- [ ] Ready for Section 13 (Reconciliation)

## Interview Questions

- "How does an HSM protect keys differently than a software wallet?"
- "Your MPC requires 3-of-5 signers. One is offline. Can you still sign?"
- "Explain why MPC is more flexible than HSM."

---

**Next**: Implement `01_hsm_interface.rs`.
