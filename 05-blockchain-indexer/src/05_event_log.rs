// FILE: src/05_event_log.rs
//
// LEARNING OBJECTIVE
// Parse event logs emitted by smart contracts.
//
// BLOCKCHAIN CONCEPT
// Events are indexed data emitted during execution and stored via logs.
//
// DESIGN DECISION
// Keep event records simple and teachable.
//
// --- IMPLEMENTATION FOLLOWS ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventLog {
    pub address: [u8; 20],
    pub topics: Vec<[u8; 32]>,
    pub data: Vec<u8>,
}

impl EventLog {
    pub fn new(address: [u8; 20], topics: Vec<[u8; 32]>, data: Vec<u8>) -> Self {
        EventLog {
            address,
            topics,
            data,
        }
    }
}

fn main() {
    let event = EventLog::new([5u8; 20], vec![[1u8; 32]], vec![9, 9, 9]);
    println!("Event from {:?}", event.address);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_event_log() {
        let event = EventLog::new([5u8; 20], vec![[1u8; 32]], vec![9, 9, 9]);
        assert_eq!(event.address, [5u8; 20]);
        assert_eq!(event.data, vec![9, 9, 9]);
    }
}
