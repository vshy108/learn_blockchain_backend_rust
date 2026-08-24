// FILE: src/06_commitment.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitmentLevel {
    Processed,
    Confirmed,
    Finalized,
}

impl CommitmentLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Processed => "processed",
            Self::Confirmed => "confirmed",
            Self::Finalized => "finalized",
        }
    }
}

fn main() {
    println!("{}", CommitmentLevel::Confirmed.as_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commitment_level_has_expected_names() {
        assert_eq!(CommitmentLevel::Processed.as_str(), "processed");
        assert_eq!(CommitmentLevel::Finalized.as_str(), "finalized");
    }
}
