// FILE: src/06_tx_broadcast.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BroadcastResult {
    pub accepted: bool,
    pub tx_hash: [u8; 32],
}

pub fn broadcast_tx(tx_hash: [u8; 32], network_ok: bool) -> BroadcastResult {
    BroadcastResult { accepted: network_ok, tx_hash }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn broadcasts_successfully_when_network_is_up() {
        let result = broadcast_tx([7u8; 32], true);
        assert!(result.accepted);
        assert_eq!(result.tx_hash, [7u8; 32]);
    }
}

fn main() {
    let res = broadcast_tx([5u8; 32], true);
    println!("accepted={}", res.accepted);
}
