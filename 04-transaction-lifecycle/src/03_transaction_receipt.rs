// FILE: src/03_transaction_receipt.rs
//
// LEARNING OBJECTIVE
// Model the result of a mined transaction: status, gas used, logs, and contract creation.
//
// BLOCKCHAIN CONCEPT
// The receipt proves execution outcome after inclusion.
//
// DESIGN DECISION
// Include key receipt fields without overcomplicating the example.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionStatus {
    Success,
    Reverted,
    OutOfGas,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Log {
    pub address: [u8; 20],
    pub topics: Vec<[u8; 32]>,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Receipt {
    pub tx_hash: [u8; 32],
    pub status: ExecutionStatus,
    pub gas_used: u64,
    pub cumulative_gas_used: u64,
    pub logs: Vec<Log>,
    pub contract_address: Option<[u8; 20]>,
}

impl Receipt {
    pub fn success(tx_hash: [u8; 32], gas_used: u64, cumulative_gas_used: u64) -> Self {
        Receipt {
            tx_hash,
            status: ExecutionStatus::Success,
            gas_used,
            cumulative_gas_used,
            logs: Vec::new(),
            contract_address: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_success_receipt() {
        let receipt = Receipt::success([1u8; 32], 21_000, 21_000);
        assert!(matches!(receipt.status, ExecutionStatus::Success));
        assert_eq!(receipt.gas_used, 21_000);
    }
}

fn main() {
    let receipt = Receipt::success([1u8; 32], 21_000, 21_000);
    println!("Receipt status: {:?}", receipt.status);
}
