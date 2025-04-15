use blake3;

#[derive(Debug)]
pub struct UninitializedPoh;

#[derive(Debug)]
pub struct InitializedPoh {
    sequence: Vec<(u64, String)>,
}

#[derive(Debug)]
pub struct VerifiedPoh {
    sequence: Vec<(u64, String)>,
}

impl UninitializedPoh {
    pub fn new() -> Self {
        UninitializedPoh
    }

    pub fn initialize(self, hash: &str, length: usize, start_time: u64, interval: u64) -> Result<InitializedPoh, String> {
        if hash.is_empty() {
            return Err("Hash cannot be empty".to_string());
        }

        if interval == 0 {
            return Err("Interval must be greater than 0".to_string());
        }
        
        let mut sequence = Vec::with_capacity(length);
        let mut current_hash = hash.to_string();

        for i in 0..length {
            let timestamp = start_time + i as u64 * interval;
            println!("Timestamp: {}", timestamp);
            current_hash = InitializedPoh::calculate_hash(&current_hash, timestamp);
            sequence.push((timestamp, current_hash.clone()));
        }
        Ok(InitializedPoh { sequence })
    }
}

impl InitializedPoh {
    fn calculate_hash(previous_hash: &str, timestamp: u64) -> String {
        let input = format!("{}{}", previous_hash, timestamp);
        let hash = blake3::hash(input.as_bytes());
        hash.to_hex().to_string()
    }

    pub fn verify(self, block_hash: &str) -> Result<VerifiedPoh, String> {
        if self.sequence.is_empty() {
            return Err("Empty PoH sequence".to_string());
        }

        let mut current_hash = block_hash.to_string();
        
        for (timestamp, stored_hash) in &self.sequence {
            let computed_hash = Self::calculate_hash(&current_hash, *timestamp);
            if computed_hash != *stored_hash {
                return Err(format!(
                    "PoH mismatch at timestamp {}: expected {}, got {}",
                    timestamp, stored_hash, computed_hash
                ));
            }
            current_hash = stored_hash.clone();
        }

        Ok(VerifiedPoh { 
            sequence: self.sequence 
        })
    }
}

impl VerifiedPoh {
    pub fn get_sequence(&self) -> &Vec<(u64, String)> {
        &self.sequence
    }

    pub fn verify_reference(&self, poh_reference: &(u64, String)) -> bool {
        let (timestamp, hash) = poh_reference;
        self.sequence
            .binary_search_by_key(timestamp, |(ts, _)| *ts)
            .map_or(false, |index| self.sequence[index].1 == *hash)
    }
}