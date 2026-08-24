// FILE: src/01_blockchain_trait.rs

// DESIGN DECISION
// The trait contains only operations shared by the supported chains. Each adapter
// can keep chain-specific details private while callers depend on one small interface.
// This is the simplest useful trait example before adding async transport or routing.
//
// RUST CONCEPTS
// - A trait defines behavior that different types can implement.
// - `impl Blockchain for EvmAdapter` connects the type to that shared behavior.
// - `&[u8]` borrows an address so the method does not take ownership of the bytes.

pub trait Blockchain {
    fn latest_block(&self) -> u64;
    fn get_balance(&self, address: &[u8]) -> u64;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvmAdapter;

impl Blockchain for EvmAdapter {
    fn latest_block(&self) -> u64 {
        12345
    }
    fn get_balance(&self, _address: &[u8]) -> u64 {
        42
    }
}

fn main() {
    let adapter = EvmAdapter;
    println!("latest={}", adapter.latest_block());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_implements_common_trait() {
        let adapter = EvmAdapter;
        assert_eq!(adapter.latest_block(), 12345);
        assert_eq!(adapter.get_balance(&[1, 2, 3]), 42);
    }
}
