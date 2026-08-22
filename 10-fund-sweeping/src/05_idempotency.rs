// FILE: src/05_idempotency.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SweepAttempt {
    pub id: u64,
    pub done: bool,
}

pub fn already_processed(id: u64, seen_ids: &[u64]) -> bool {
    seen_ids.contains(&id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_duplicate_sweep_attempts() {
        let seen = [7, 9, 11];
        assert!(already_processed(9, &seen));
        assert!(!already_processed(8, &seen));
    }
}

fn main() {
    println!("{}", already_processed(5, &[1, 2, 5]));
}
