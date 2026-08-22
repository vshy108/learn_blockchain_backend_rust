// FILE: src/01_utxo.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Outpoint {
    pub tx_id: [u8; 32],
    pub index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UTXO {
    pub outpoint: Outpoint,
    pub amount: u64,
    pub script_pubkey: Vec<u8>,
}

impl UTXO {
    pub fn new(tx_id: [u8; 32], index: u32, amount: u64, script_pubkey: Vec<u8>) -> Self {
        Self {
            outpoint: Outpoint { tx_id, index },
            amount,
            script_pubkey,
        }
    }

    pub fn is_spendable(&self, lock_script: &[u8]) -> bool {
        self.script_pubkey == lock_script
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_utxo() {
        let utxo = UTXO::new([9u8; 32], 0, 50_000_000, vec![0x51, 0x02]);
        assert_eq!(utxo.amount, 50_000_000);
        assert_eq!(utxo.outpoint.index, 0);
    }

    #[test]
    fn recognizes_matching_script() {
        let utxo = UTXO::new([1u8; 32], 2, 10_000, vec![0xaa, 0xbb]);
        assert!(utxo.is_spendable(&[0xaa, 0xbb]));
    }
}

fn main() {
    let utxo = UTXO::new([1u8; 32], 1, 3_000_000, vec![0x51]);
    println!("UTXO amount: {} sat", utxo.amount);
}
