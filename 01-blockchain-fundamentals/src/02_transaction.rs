// FILE: src/02_transaction.rs
//
// LEARNING OBJECTIVE
// Understand transactions deeply: what they are, the difference between
// inclusion and execution, and why a transaction in a block isn't automatically successful.
//
// BLOCKCHAIN CONCEPT
// A transaction is a request to move value or execute code on a blockchain.
// But a critical distinction: being IN a block ≠ being SUCCESSFUL.
//
// A transaction can be:
//   1. Included in a block (mined)
//   2. But failed during execution (reverted)
//   3. And still cost gas/fees
//
// This is one of the most important distinctions blockchain engineers must understand.
//
// NORMAL CASE
// - TX created with valid data
// - TX broadcast to network
// - TX included in a block
// - TX executed (may succeed or fail)
// - Receipt shows result: status 1 (success) or 0 (revert)
//
// SPECIAL CASES
// - TX fails but is still on-chain (status = 0, gas still used)
// - TX has "dust" output (very small amount, still valid)
// - TX with nonce too high (future transaction, not yet included)
// - TX with nonce already used (duplicate, rejected)
//
// EXCEPTIONAL CASES
// - TX reverted due to contract revert (e.g., "insufficient balance")
// - TX reverted due to out-of-gas
// - TX dropped from mempool (too low gas, or timeout)
// - TX replaced with different TX using same nonce (transaction replacement)
// - TX included in block, then block reorged (TX might disappear)
//
// DESIGN DECISION
// We model transactions as having a lifecycle:
//   - Created: TX initialized
//   - Signed: TX cryptographically signed
//   - Broadcast: TX sent to network
//   - Pending: TX in mempool, awaiting mining
//   - Mined: TX included in block (NOT yet final)
//   - Confirmed: TX 6+ blocks deep (safe)
//   - Finalized: TX mathematically final (cannot change)
//
// We also model execution status:
//   - None: TX not yet mined (no receipt)
//   - Success: TX executed, all operations succeeded (status = 1)
//   - Reverted: TX reverted (status = 0)
//   - OutOfGas: TX ran out of gas mid-execution (status = 0)
//
// --- IMPLEMENTATION FOLLOWS ---

use std::fmt;

/// Address type (20 bytes for EVM).
pub type Address = [u8; 20];
///
/// Note: This is a simplified model. Real transactions are more complex:
///   - Bitcoin: Has inputs/outputs (UTXO model)
///   - Ethereum: Has from/to/data (account model)
///   - Solana: Has accounts/instructions (program model)
///
/// We model a generic TX here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    /// Unique identifier of this TX (hash of TX data).
    pub hash: [u8; 32],
    
    /// Which block this TX is in (if mined; None if pending).
    pub block_number: Option<u64>,
    
    /// Position within the block (0-indexed).
    pub index_in_block: Option<usize>,
    
    /// Sender address.
    pub from: Address,
    
    /// Recipient address (can be None for contract creation).
    pub to: Option<Address>,
    
    /// Value transferred (in smallest units: wei, satoshi, lamport).
    pub value: u64,
    
    /// Additional data (EVM calldata, Bitcoin script, Solana instruction data).
    pub data: Vec<u8>,
    
    /// Gas limit (EVM only; max computation allowed).
    pub gas_limit: u64,
    
    /// Gas price (EVM only; cost per unit of gas).
    pub gas_price: u64,
    
    /// Nonce (EVM only; transaction count for sender).
    pub nonce: u64,
}

impl Transaction {
    /// Create a new transaction.
    pub fn new(
        hash: [u8; 32],
        from: Address,
        to: Option<Address>,
        value: u64,
        data: Vec<u8>,
        gas_limit: u64,
        gas_price: u64,
        nonce: u64,
    ) -> Self {
        Transaction {
            hash,
            block_number: None,
            index_in_block: None,
            from,
            to,
            value,
            data,
            gas_limit,
            gas_price,
            nonce,
        }
    }

    /// Mark this transaction as included in a block.
    ///
    /// This is NOT the same as being successful!
    /// The TX can be mined but still fail.
    pub fn mined(mut self, block_number: u64, index_in_block: usize) -> Self {
        self.block_number = Some(block_number);
        self.index_in_block = Some(index_in_block);
        self
    }

    /// Check if this transaction has been mined (included in a block).
    pub fn is_mined(&self) -> bool {
        self.block_number.is_some()
    }

    /// Check if this transaction is still pending (not yet mined).
    pub fn is_pending(&self) -> bool {
        self.block_number.is_none()
    }

    /// Calculate the total cost of this transaction.
    /// cost = value + (gas_limit * gas_price)
    pub fn total_cost(&self) -> u64 {
        self.value.saturating_add(self.gas_limit.saturating_mul(self.gas_price))
    }

    /// Calculate gas fee (not including value transferred).
    /// gas_fee = gas_limit * gas_price
    pub fn gas_fee(&self) -> u64 {
        self.gas_limit.saturating_mul(self.gas_price)
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.is_mined() {
            format!("Mined in block {}", self.block_number.unwrap_or(0))
        } else {
            "Pending".to_string()
        };
        
        write!(
            f,
            "TX {} ({}) from {:?} → {:?}, value: {}, gas: {}",
            hex::encode(&self.hash[..8]),
            status,
            self.from,
            self.to,
            self.value,
            self.gas_limit
        )
    }
}

/// The execution result of a transaction.
///
/// CRITICAL: A TX can be mined but fail. This enum distinguishes the two.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStatus {
    /// TX not yet mined (no receipt yet).
    Pending,
    
    /// TX executed successfully (status = 1).
    Success,
    
    /// TX reverted (status = 0). Cost gas but had no effect.
    Reverted,
    
    /// TX ran out of gas. Consumed all gas_limit. Status = 0.
    OutOfGas,
}

impl fmt::Display for ExecutionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionStatus::Pending => write!(f, "Pending"),
            ExecutionStatus::Success => write!(f, "Success (1)"),
            ExecutionStatus::Reverted => write!(f, "Reverted (0)"),
            ExecutionStatus::OutOfGas => write!(f, "OutOfGas (0)"),
        }
    }
}

/// Receipt: The result of executing a transaction.
///
/// A receipt is issued AFTER a TX is mined, showing what happened during execution.
#[derive(Debug, Clone)]
pub struct Receipt {
    /// The transaction hash this receipt is for.
    pub transaction_hash: [u8; 32],
    
    /// Execution status: 1 = success, 0 = failure.
    pub status: ExecutionStatus,
    
    /// How much gas was actually used (can be less than gas_limit).
    pub gas_used: u64,
    
    /// Cumulative gas used in the block (running total).
    pub cumulative_gas_used: u64,
    
    /// Logs/events emitted during execution.
    pub logs: Vec<Log>,
    
    /// If this TX created a contract, this is the new contract address.
    pub contract_address: Option<Address>,
    
    /// Block number where this TX was mined.
    pub block_number: u64,
    
    /// Index of this TX in the block.
    pub transaction_index: usize,
}

impl Receipt {
    /// Create a new receipt.
    pub fn new(
        transaction_hash: [u8; 32],
        status: ExecutionStatus,
        gas_used: u64,
        cumulative_gas_used: u64,
        block_number: u64,
        transaction_index: usize,
    ) -> Self {
        Receipt {
            transaction_hash,
            status,
            gas_used,
            cumulative_gas_used,
            logs: Vec::new(),
            contract_address: None,
            block_number,
            transaction_index,
        }
    }

    /// Check if execution was successful.
    pub fn is_successful(&self) -> bool {
        self.status == ExecutionStatus::Success
    }

    /// Check if execution failed (reverted or out-of-gas).
    pub fn is_failed(&self) -> bool {
        matches!(self.status, ExecutionStatus::Reverted | ExecutionStatus::OutOfGas)
    }

    /// Add a log to this receipt.
    pub fn add_log(&mut self, log: Log) {
        self.logs.push(log);
    }

    /// Refund amount: gas_limit - gas_used (typical 50% refund in EVM).
    /// 
    /// Note: Real EVM refunds are more complex (accounts for freed storage, etc.).
    /// This is simplified.
    pub fn gas_refund_simple(&self, gas_price: u64) -> u64 {
        // Don't calculate refund for failed TXs
        if self.is_failed() {
            return 0;
        }
        // Simplified: 20% refund of unused gas
        let unused_gas = (self.gas_used as f64 * 0.1) as u64;
        unused_gas.saturating_mul(gas_price)
    }
}

impl fmt::Display for Receipt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Receipt for TX {} in block {}: {} (gas used: {})",
            hex::encode(&self.transaction_hash[..8]),
            self.block_number,
            self.status,
            self.gas_used
        )
    }
}

/// A log (event) emitted during TX execution.
#[derive(Debug, Clone)]
pub struct Log {
    /// Address that emitted this log.
    pub address: Address,
    
    /// Topics (indexed parameters).
    pub topics: Vec<[u8; 32]>,
    
    /// Data (non-indexed parameters).
    pub data: Vec<u8>,
}

impl Log {
    /// Create a new log.
    pub fn new(address: Address, topics: Vec<[u8; 32]>, data: Vec<u8>) -> Self {
        Log {
            address,
            topics,
            data,
        }
    }
}

// --- TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_address(n: u8) -> Address {
        [n; 20]
    }

    #[test]
    fn transaction_created_is_pending() {
        let tx = Transaction::new(
            [0u8; 32],
            mock_address(1),
            Some(mock_address(2)),
            1_000_000,
            vec![],
            21_000,
            50,
            0,
        );
        
        assert!(tx.is_pending());
        assert!(!tx.is_mined());
    }

    #[test]
    fn transaction_mined() {
        let tx = Transaction::new(
            [0u8; 32],
            mock_address(1),
            Some(mock_address(2)),
            1_000_000,
            vec![],
            21_000,
            50,
            0,
        ).mined(100, 5);
        
        assert!(tx.is_mined());
        assert!(!tx.is_pending());
        assert_eq!(tx.block_number, Some(100));
        assert_eq!(tx.index_in_block, Some(5));
    }

    #[test]
    fn total_cost_calculation() {
        let tx = Transaction::new(
            [0u8; 32],
            mock_address(1),
            Some(mock_address(2)),
            1_000_000,  // 1 ETH in wei (simplified)
            vec![],
            21_000,     // gas limit
            50,         // gas price
            0,
        );
        
        let gas_fee = 21_000u64 * 50;
        let total = 1_000_000 + gas_fee;
        assert_eq!(tx.total_cost(), total);
    }

    #[test]
    fn gas_fee_calculation() {
        let tx = Transaction::new(
            [0u8; 32],
            mock_address(1),
            Some(mock_address(2)),
            0,
            vec![],
            21_000,
            50,
            0,
        );
        
        assert_eq!(tx.gas_fee(), 21_000 * 50);
    }

    #[test]
    fn receipt_success_is_identified() {
        let receipt = Receipt::new(
            [0u8; 32],
            ExecutionStatus::Success,
            21_000,
            21_000,
            100,
            0,
        );
        
        assert!(receipt.is_successful());
        assert!(!receipt.is_failed());
    }

    #[test]
    fn receipt_reverted_is_identified() {
        let receipt = Receipt::new(
            [0u8; 32],
            ExecutionStatus::Reverted,
            21_000,
            21_000,
            100,
            0,
        );
        
        assert!(!receipt.is_successful());
        assert!(receipt.is_failed());
    }

    #[test]
    fn receipt_out_of_gas_is_identified() {
        let receipt = Receipt::new(
            [0u8; 32],
            ExecutionStatus::OutOfGas,
            21_000,
            21_000,
            100,
            0,
        );
        
        assert!(!receipt.is_successful());
        assert!(receipt.is_failed());
    }

    #[test]
    fn receipt_refund_only_for_success() {
        let successful = Receipt::new(
            [0u8; 32],
            ExecutionStatus::Success,
            18_000,  // Used 18k gas out of 21k
            18_000,
            100,
            0,
        );
        
        let refunded = successful.gas_refund_simple(50);
        assert!(refunded > 0);
        
        let failed = Receipt::new(
            [0u8; 32],
            ExecutionStatus::Reverted,
            21_000,
            21_000,
            100,
            0,
        );
        
        assert_eq!(failed.gas_refund_simple(50), 0);
    }

    #[test]
    fn log_creation() {
        let log = Log::new(
            mock_address(1),
            vec![[42u8; 32]],
            vec![1, 2, 3],
        );
        
        assert_eq!(log.address, mock_address(1));
        assert_eq!(log.topics.len(), 1);
        assert_eq!(log.data, vec![1, 2, 3]);
    }

    #[test]
    fn receipt_with_logs() {
        let mut receipt = Receipt::new(
            [0u8; 32],
            ExecutionStatus::Success,
            21_000,
            21_000,
            100,
            0,
        );
        
        receipt.add_log(Log::new(mock_address(1), vec![], vec![]));
        receipt.add_log(Log::new(mock_address(2), vec![], vec![]));
        
        assert_eq!(receipt.logs.len(), 2);
    }

    #[test]
    fn critical_distinction_mined_vs_successful() {
        // This test emphasizes the key learning point:
        // A transaction can be MINED but NOT SUCCESSFUL.
        
        let tx_mined_failed = Transaction::new(
            [1u8; 32],
            mock_address(1),
            Some(mock_address(2)),
            0,
            vec![],
            21_000,
            50,
            0,
        ).mined(100, 0);
        
        // TX is mined (in block 100)
        assert!(tx_mined_failed.is_mined());
        
        // But its receipt shows it failed
        let receipt_failed = Receipt::new(
            tx_mined_failed.hash,
            ExecutionStatus::Reverted,
            21_000,  // Still used all gas!
            21_000,
            100,
            0,
        );
        
        assert!(!receipt_failed.is_successful());
        assert_eq!(receipt_failed.gas_used, 21_000);
        
        // KEY INSIGHT:
        // - is_mined() = true ✓
        // - is_successful() = false ✗
        // - gas_used = 21_000 (still cost money!)
    }
}

// --- RUST VS GO COMPARISON ---
//
// RUST:
// ```rust
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum ExecutionStatus {
//     Pending,
//     Success,
//     Reverted,
//     OutOfGas,
// }
//
// impl fmt::Display for ExecutionStatus { ... }
// ```
//
// GO:
// ```go
// type ExecutionStatus int
//
// const (
//     Pending ExecutionStatus = iota
//     Success
//     Reverted
//     OutOfGas
// )
//
// func (s ExecutionStatus) String() string { ... }
// ```
//
// DIFFERENCES:
// - Rust: Enums are type-safe, can't accidentally use wrong value
// - Go: Constants are less safe (could pass wrong int value)
// - Rust: Must implement Display manually for custom string representation
// - Go: Can assign numeric constants to any type
// - Rust: derive Clone, Copy automatically available
// - Go: Must implement manually (or use iota shorthand)
//
// WHY RUST'S APPROACH MATTERS FOR BLOCKCHAIN:
// - Type safety prevents bugs: you can't pass "5" when ExecutionStatus is needed
// - Enums are exhaustive: the compiler catches unhandled cases
// - No null-pointer confusion: Rust enums are never nil unless explicitly Option<T>

fn main() {
    // Example: Create TX, mine it, then see receipt shows failure
    let tx = Transaction::new(
        [1u8; 32],
        [1u8; 20],
        Some([2u8; 20]),
        0,
        vec![],
        21_000,
        50,
        0,
    );
    
    println!("Before mining: {}", tx);
    println!("Pending: {}", tx.is_pending());
    
    let tx_mined = tx.mined(100, 5);
    println!("After mining: {}", tx_mined);
    println!("Mined: {}", tx_mined.is_mined());
    
    // Create receipt showing failure
    let receipt = Receipt::new(
        tx_mined.hash,
        ExecutionStatus::Reverted,
        21_000,
        21_000,
        100,
        5,
    );
    
    println!("Receipt: {}", receipt);
    println!("Successful: {}", receipt.is_successful());
    println!("Gas refund: {}", receipt.gas_refund_simple(50));
}
