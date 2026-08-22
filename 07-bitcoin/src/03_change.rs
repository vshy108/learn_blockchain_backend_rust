// FILE: src/03_change.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeCalculation {
    pub input_total: u64,
    pub output_total: u64,
    pub fee: u64,
}

pub fn compute_change(input_total: u64, output_total: u64) -> ChangeCalculation {
    let fee = input_total.saturating_sub(output_total);
    ChangeCalculation { input_total, output_total, fee }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_change_and_fee() {
        let change = compute_change(10_000, 9_200);
        assert_eq!(change.fee, 800);
        assert_eq!(change.output_total, 9_200);
    }
}

fn main() {
    let change = compute_change(10_000, 8_500);
    println!("fee={}", change.fee);
}
