use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u32,
}

impl Block {
    fn new(index: u32, data: String, previous_hash: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error getting current time")
            .as_millis();
        let nonce = 0; // Initial nonce
        let hash = calculate_hash( index, &previous_hash, timestamp, &data, nonce );
        Block { index, timestamp, data, previous_hash, hash, nonce }
    }

    fn calculate_hash(index: u32, previous_hash: &str, timestamp: u64, data: &str, nonce: u32) -> String {
        let input = format!("{}{}{}{}{}", index, previous_hash, timestamp, data, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}