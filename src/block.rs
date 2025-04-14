use std::time::{SystemTime, UNIX_EPOCH};
use blake3;
use serde::{Serialize, Deserialize};
use crate::poh::Poh;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u32,
    poh_sequence: Vec<(u64, String)>,
}

impl Block {
    pub fn new(index: u32, data: String, previous_hash: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error getting current time")
            .as_millis() as u64;
        let nonce = 0; // Initial nonce
        let hash = Self::calculate_hash(index, &previous_hash, timestamp, &data, nonce);
        let interval = 1;
        let poh_sequence = Poh::new(&hash, 5, timestamp, interval); // 5 hashes in the sequence

        println!("poh_sequence: {:?}", poh_sequence);

        Block {index, timestamp, data, previous_hash, hash, nonce, poh_sequence}
    }

    fn calculate_hash(index: u32, previous_hash: &str, timestamp: u64, data: &str, nonce: u32) -> String {
        let input = format!("{}{}{}{}{}", index, previous_hash, timestamp, data, nonce);
        let hash = blake3::hash(input.as_bytes());
        hash.to_hex().to_string()
    }

    pub fn verify(&self) -> bool {
        let poh = Poh { poh_sequence: self.poh_sequence.clone() };
        poh.verify_poh_sequence(&self.hash) // Pass the block's hash to the POH verification
    }
}