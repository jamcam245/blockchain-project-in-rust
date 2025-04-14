use serde::{Serialize, Deserialize};
use blake3;


#[derive(Debug, Serialize, Deserialize)]
pub struct Poh {
    pub poh_sequence: Vec<(u64, String)>,
}

impl Poh {
    pub fn new(hash: &str, length: usize, start_time: u64, interval: u64 ) -> Vec<(u64, String)> {
        let mut sequence = Vec::new();
        let mut current_hash = hash.to_string();

        for i in 0..length {
            let timestamp = start_time + i as u64 * interval;
            current_hash = Self::calculate_hash(0, &current_hash, timestamp, "", 0);
            sequence.push((timestamp, current_hash.clone()));
        }

        sequence


    }

    pub fn verify_poh_sequence(&self, block_hash: &str) -> bool {
        if self.poh_sequence.is_empty() {
            return false; // Sequence is empty, invalid POH sequence
        }
    
        let mut current_hash = block_hash; // Start with the block's hash
    
        for (timestamp, stored_hash) in &self.poh_sequence {
            // Recompute the hash using the current hash and timestamp from the sequence
            let recomputed_hash = Self::calculate_hash(0, &current_hash, *timestamp, "", 0);
    
            // Check if the recomputed hash matches the stored hash in the sequence
            // If they don't match, the sequence is invalid
            if recomputed_hash != *stored_hash {
                return false; // Sequence is invalid
            }
    
            // Update the current hash for the next iteration
            current_hash = stored_hash;
        }
    
        true // Sequence is valid
    }

    pub fn calculate_hash(index: u32, previous_hash: &str, timestamp: u64, data: &str, nonce: u32) -> String {
        let input = format!("{}{}{}{}{}", index, previous_hash, timestamp, data, nonce);
        let hash = blake3::hash(input.as_bytes());
        hash.to_hex().to_string()
    }
}