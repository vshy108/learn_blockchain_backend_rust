# Section 14: Proof of Reserves

## Goal

Prove customer assets are backed by blockchain holdings without exposing customer data.

## Why This Section

After FTX collapse, exchanges must prove reserves. This section teaches the mathematical approach: **Merkle trees and cryptographic proofs**.

## The Challenge

```
Exchange has 1 million customers.
Total customer liabilities: $500M
Exchange wants to prove: "We have $500M+ on blockchain"

But: Cannot publish every customer's balance (privacy + security risk).

Solution: Merkle tree commitment.
```

## The Approach

```
Customer 1: $100k
Customer 2: $50k
...
Customer 1M: $200k

Build Merkle tree:
           Root (publish this)
          /    \
       Node1    Node2
       /  \      /  \
     C1 C2 C3 C4
     
Proof for Customer 1:
  "My balance: $100k, ID: 1"
  "Hash: XXX"
  "This hashes to Root if you also have: C2, Node2"
  
Verifier can check:
  hash(hash($100k + 1) + hash(C2)) = Node1
  hash(Node1 + Node2) = Root ✓
  
Without seeing any other customer's balance.
```

## Concepts You'll Learn

| Concept | Why It Matters |
|---------|---|
| Merkle tree | Efficient commitment to large sets |
| Merkle proof | Proving inclusion without revealing set |
| Hash functions | One-way functions |
| Commitment | Public, binding, can't change later |

## Files You'll Create

1. `01_merkle_tree.rs` — Building a merkle tree
2. `02_merkle_proof.rs` — Generating proofs
3. `03_merkle_verify.rs` — Verifying proofs
4. `04_commitment.rs` — Commitment scheme
5. `05_liability_tree.rs` — Customer balance tree
6. `06_proof_report.rs` — Full PoR report

## Data Structures

```rust
pub struct MerkleTree {
    pub leaves: Vec<[u8; 32]>,
    pub nodes: Vec<Vec<[u8; 32]>>,  // tree structure
}

pub struct MerkleProof {
    pub leaf_index: usize,
    pub leaf_hash: [u8; 32],
    pub path: Vec<[u8; 32]>,  // hashes needed to verify
}
```

## Running Tests

```bash
cargo test --package 14-proof-of-reserves
```

## Acceptance Criteria

- [ ] Can build a merkle tree from customer balances
- [ ] Generate proof for any customer
- [ ] Verify proof without seeing other customers
- [ ] Root hash published and audited
- [ ] Proof report is comprehensive
- [ ] Ready for Section 15 (Production)

## Interview Questions

- "Design a proof-of-reserves system. How do you prevent a customer from double-counting their balance?"
- "Can a Merkle proof lie?"
- "Why can't you just hash all customer balances and publish that hash?"

---

**Next**: Implement `01_merkle_tree.rs`.
