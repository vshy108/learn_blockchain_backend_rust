// FILE: src/06_sweep_tracker.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SweepTracker {
    pub attempts: Vec<u64>,
}

impl SweepTracker {
    pub fn new() -> Self {
        Self {
            attempts: Vec::new(),
        }
    }

    pub fn record(&mut self, id: u64) {
        self.attempts.push(id);
    }
}

impl Default for SweepTracker {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut tracker = SweepTracker::new();
    tracker.record(1);
    println!("attempts={}", tracker.attempts.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_attempt_history() {
        let mut tracker = SweepTracker::new();
        tracker.record(10);
        tracker.record(20);
        assert_eq!(tracker.attempts, vec![10, 20]);
    }
}
