# Section 03: EVM Client Layer

## Goal

Build a high-level EVM client that abstracts RPC protocol details away from the application.

## Why This Section

After Section 02, you can construct RPC calls. But application code should never deal with JSON and hexadecimal directly. This section builds a **clean abstraction layer**.

## Architecture

```
Application Code
      ↓
EVM Client (you build this)
      ↓
RPC Client (Section 02)
      ↓
HTTP
      ↓
EVM Node
```

## Concepts You'll Learn

| Concept | Why It Matters |
|---------|---|
| Trait-based abstraction | Testable, swappable implementations |
| Error types | Structured errors instead of strings |
| Async Rust | RPC calls are I/O bound |
| Type safety | Hex strings → u64, not strings throughout |

## Files You'll Create

1. `01_evm_client.rs` — Client struct and basic interface
2. `02_latest_block.rs` — `async fn latest_block()`
3. `03_get_block_details.rs` — Full block information
4. `04_account_state.rs` — Balance, nonce, code queries
5. `05_error_handling.rs` — Structured error types
6. `06_client_tests.rs` — Client-level integration tests

## Key Design Decisions

### Error Handling
```rust
pub enum ClientError {
    RpcError { code: i64, message: String },
    NetworkError(String),
    ParseError(String),
}
```
Not just `Result<T, String>`.

### Async Methods
```rust
pub async fn latest_block(&self) -> Result<u64, ClientError>
```
RPC calls are I/O, so they're async.

### Type Safety
**Bad**:
```rust
let block = client.get_block("0x15")?;  // what type is block?
```

**Good**:
```rust
let block: Block = client.get_block(21)?; // u64 in, Block out
```

## Running Tests

```bash
cargo test --package 03-evm-client
```

## Acceptance Criteria

- [ ] Client abstracts RPC format from callers
- [ ] All async methods use `tokio::spawn` or similar
- [ ] Errors distinguish node errors from local errors
- [ ] Type safety: no raw JSON strings in public API
- [ ] Ready for Section 04 (Transaction Lifecycle)

## Interview Questions

- "Design an EVM client abstraction. What methods must it have?"
- "How do you test a client that makes HTTP calls without hitting a real node?"
- "What error types does your client expose, and why?"

---

**Next**: Implement `01_evm_client.rs`.
