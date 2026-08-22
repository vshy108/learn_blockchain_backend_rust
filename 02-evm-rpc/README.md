# Section 02: EVM RPC Protocol

## Goal

Learn how to communicate with Ethereum nodes using JSON-RPC 2.0. Understand the protocol structure, error handling, and hexadecimal encoding.

## Why This Section

Your Rust code will never touch blocks or transactions directly. It will always ask an **RPC node** for this data. Understanding the protocol is critical for debugging and handling edge cases.

## Concepts You'll Learn

| Concept | Why It Matters |
|---------|---|
| JSON-RPC 2.0 | Universal blockchain RPC format |
| Request structure | method, params, id |
| Hexadecimal encoding | `0x...` format for EVM numbers |
| RPC errors | Distinguish from transport errors |
| Request ID | Matching requests to responses |

## Files You'll Create

1. `01_rpc_request.rs` — Building JSON-RPC requests
2. `02_rpc_response.rs` — Parsing JSON-RPC responses
3. `03_hex_encoding.rs` — Converting hex ↔ u64
4. `04_block_number.rs` — `eth_blockNumber` call
5. `05_get_block.rs` — `eth_getBlockByNumber` call
6. `06_get_code.rs` — `eth_getCode` call

## Key Distinctions

### Request Format
```json
{
  "jsonrpc": "2.0",
  "method": "eth_blockNumber",
  "params": [],
  "id": 1
}
```
- **jsonrpc**: Always "2.0"
- **method**: RPC method name (e.g., "eth_blockNumber")
- **params**: Array of parameters (order matters)
- **id**: Unique ID to match request → response

### Success Response
```json
{
  "jsonrpc": "2.0",
  "result": "0x15",
  "id": 1
}
```
- **result**: The return value (could be null)
- **id**: Matches the request

### Error Response
```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32601,
    "message": "Method not found"
  },
  "id": 1
}
```
- **error**: Object with code + message
- **result**: Not present in error responses

### Hexadecimal Encoding
- All EVM numbers: `0x...` (hex notation)
- `0x0` = 0
- `0xff` = 255
- `0x100` = 256
- **Challenge**: Parse hex strings to Rust integers without errors

## Running Tests

```bash
cargo test --package 02-evm-rpc
cargo test --package 02-evm-rpc -- --nocapture
```

## Acceptance Criteria

After completing this section:

- [ ] Can construct valid JSON-RPC requests
- [ ] Parse JSON-RPC responses and errors correctly
- [ ] Convert hex ↔ u64 safely
- [ ] Understand the 6 key EVM RPC methods
- [ ] Ready for Section 03 (EVM Client Layer)

## Interview Questions

- "What's the difference between a JSON-RPC error and an HTTP error?"
- "If you get `0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff` back from an RPC call, how do you safely parse it to u256?"
- "Write the JSON-RPC request for `eth_getBalance` for address `0x742d35Cc...` at the latest block."

---

**Next**: Implement `01_rpc_request.rs`.
