// FILE: src/03_evm_adapter.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvmAdapter {
    pub latest_block: u64,
}

impl EvmAdapter {
    pub fn new(latest_block: u64) -> Self {
        Self { latest_block }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evm_adapter_tracks_block() {
        let adapter = EvmAdapter::new(42);
        assert_eq!(adapter.latest_block, 42);
    }
}

fn main() {
    let adapter = EvmAdapter::new(10);
    println!("block={}", adapter.latest_block);
}
