// FILE: src/02_bitcoin_adapter.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitcoinAdapter {
    pub best_height: u64,
}

impl BitcoinAdapter {
    pub fn new(best_height: u64) -> Self {
        Self { best_height }
    }
}

fn main() {
    let adapter = BitcoinAdapter::new(999);
    println!("height={}", adapter.best_height);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitcoin_adapter_tracks_height() {
        let adapter = BitcoinAdapter::new(1200);
        assert_eq!(adapter.best_height, 1200);
    }
}
