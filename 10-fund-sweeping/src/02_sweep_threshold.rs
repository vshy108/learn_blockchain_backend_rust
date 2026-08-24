// FILE: src/02_sweep_threshold.rs

pub fn should_sweep(balance: u64, threshold: u64) -> bool {
    balance > threshold
}

fn main() {
    println!("{}", should_sweep(30, 20));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_sweeps_above_threshold() {
        assert!(should_sweep(100, 50));
        assert!(!should_sweep(50, 50));
    }
}
