// FILE: src/04_fee_calculation.rs

pub fn fee_for_transaction(input_total: u64, output_total: u64) -> u64 {
    input_total.saturating_sub(output_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_fee() {
        assert_eq!(fee_for_transaction(1_000_000, 980_000), 20_000);
    }
}

fn main() {
    println!("fee={}", fee_for_transaction(100, 80));
}
