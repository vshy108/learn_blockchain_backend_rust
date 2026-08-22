// FILE: src/05_routing.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainKind {
    Bitcoin,
    Evm,
    Solana,
}

pub fn route_chain(kind: ChainKind) -> &'static str {
    match kind {
        ChainKind::Bitcoin => "bitcoin",
        ChainKind::Evm => "evm",
        ChainKind::Solana => "solana",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_known_chain_types() {
        assert_eq!(route_chain(ChainKind::Bitcoin), "bitcoin");
        assert_eq!(route_chain(ChainKind::Solana), "solana");
    }
}

fn main() {
    println!("{}", route_chain(ChainKind::Evm));
}
