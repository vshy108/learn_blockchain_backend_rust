// FILE: src/02_program.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramAddress {
    pub address: [u8; 32],
    pub seed: Vec<u8>,
}

pub fn derive_program_address(seed: &[u8]) -> ProgramAddress {
    let mut hash = [0u8; 32];
    for (i, byte) in seed.iter().enumerate() {
        hash[i % 32] = hash[i % 32].wrapping_add(*byte);
    }
    ProgramAddress {
        address: hash,
        seed: seed.to_vec(),
    }
}

fn main() {
    let pda = derive_program_address(b"demo");
    println!("pda={:?}", pda.address);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_program_address_from_seed() {
        let address = derive_program_address(b"vault");
        assert_eq!(address.seed, b"vault".to_vec());
        assert_eq!(address.address.len(), 32);
    }
}
