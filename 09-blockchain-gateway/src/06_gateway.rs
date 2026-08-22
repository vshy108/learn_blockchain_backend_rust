// FILE: src/06_gateway.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gateway {
    pub supported_chains: Vec<&'static str>,
}

impl Gateway {
    pub fn new() -> Self {
        Self { supported_chains: vec!["bitcoin", "evm", "solana"] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gateway_supports_all_expected_chains() {
        let gateway = Gateway::new();
        assert_eq!(gateway.supported_chains.len(), 3);
        assert!(gateway.supported_chains.contains(&"evm"));
    }
}

fn main() {
    let gateway = Gateway::new();
    println!("chains={}", gateway.supported_chains.len());
}
