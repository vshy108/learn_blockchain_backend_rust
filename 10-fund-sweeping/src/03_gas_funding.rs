// FILE: src/03_gas_funding.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GasFundingPlan {
    pub native_needed: u64,
    pub token_to_transfer: u64,
}

pub fn plan_gas_funding(balance: u64, token_amount: u64, gas_needed: u64) -> GasFundingPlan {
    let native_needed = if balance >= gas_needed { 0 } else { gas_needed - balance };
    GasFundingPlan { native_needed, token_to_transfer: token_amount }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plans_funding_when_native_balance_is_insufficient() {
        let plan = plan_gas_funding(0, 1_000, 50);
        assert_eq!(plan.native_needed, 50);
    }
}

fn main() {
    let plan = plan_gas_funding(20, 80, 50);
    println!("native_needed={}", plan.native_needed);
}
