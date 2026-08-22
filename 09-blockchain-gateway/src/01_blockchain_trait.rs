// FILE: src/01_blockchain_trait.rs

pub trait Blockchain {
    fn latest_block(&self) -> u64;
    fn get_balance(&self, address: &[u8]) -> u64;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvmAdapter;

impl Blockchain for EvmAdapter {
    fn latest_block(&self) -> u64 { 12345 }
    fn get_balance(&self, _address: &[u8]) -> u64 { 42 }
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

fn main() {
    let adapter = EvmAdapter;
    println!("latest={}", adapter.latest_block());
}
