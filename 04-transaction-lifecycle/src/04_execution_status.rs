// FILE: src/04_execution_status.rs
//
// LEARNING OBJECTIVE
// Separate mined from successful. This is the key mental model for backend work.
//
// BLOCKCHAIN CONCEPT
// A mined tx may revert or run out of gas but still appear on-chain.
//
// DESIGN DECISION
// Model the execution result explicitly.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStatus {
    Pending,
    Success,
    Reverted,
    OutOfGas,
}

impl ExecutionStatus {
    pub fn is_successful(&self) -> bool {
        matches!(self, ExecutionStatus::Success)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinguishes_success_from_revert() {
        assert!(ExecutionStatus::Success.is_successful());
        assert!(!ExecutionStatus::Reverted.is_successful());
    }
}

fn main() {
    println!("Success? {}", ExecutionStatus::Success.is_successful());
}
