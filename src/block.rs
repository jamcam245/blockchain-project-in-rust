use blake3;
use serde::{Serialize, Deserialize};
use crate::transaction::ValidatedTransaction;
use crate::poh::VerifiedPoh;

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingBlock {
    previous_hash : String,
    transactions : Vec<ValidatedTransaction>,
    max_transactions : usize,
    timestamp : u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommittedBlock {
    index: u32,
    timestamp: u64,
    previous_hash: String,
    hash: String,
    poh_reference: (u64, String),
    transactions: Vec<ValidatedTransaction>,
}

impl PendingBlock {

    pub fn new(previous_hash: String, max_transactions: usize) -> Self {
        PendingBlock {
            previous_hash,
            transactions: Vec::new(),
            max_transactions,
            timestamp: 0,
        }
    }
        
    pub fn finalize(self, index: u32, poh_reference: (u64, String)) -> CommittedBlock {
        let timestamp = poh_reference.0;

        let hash = CommittedBlock::calculate_hash(index, &self.previous_hash, timestamp, &self.transactions);

        CommittedBlock {
            index,
            previous_hash: self.previous_hash,
            transactions: self.transactions,
            timestamp,
            hash, 
            poh_reference,
        }
    }

    pub fn add_transaction(&mut self, transaction: ValidatedTransaction) {
        if self.transactions.len() < self.max_transactions {
            self.transactions.push(transaction);
        } else {
            println!("Max transactions reached for this block.");
        }
    }
}


impl CommittedBlock {
    pub fn verify(&self, poh: &VerifiedPoh) -> bool {
        // Verify the PoH reference
        if !poh.verify_reference(&self.poh_reference) {
            println!("PoH reference is invalid!");
            return false;
        }

        // Cross-validate the timestamp with the PoH reference
        let (tick, _) = self.poh_reference;
        if let Some((expected_tick, _)) = poh.get_sequence().iter().find(|(t, _)| *t == tick) {
            if *expected_tick != tick {
                println!("Timestamp does not match PoH reference!");
                return false;
            }
        }

        let calculated_hash = Self::calculate_hash(self.index, &self.previous_hash, self.timestamp, &self.transactions);

        println!("Calculated hash: {}", calculated_hash);
        println!("Block hash: {}", self.hash);
        if self.hash != calculated_hash {
            println!("Block hash is invalid!");
            return false;
        }

        true
    }

    fn calculate_hash(index: u32, previous_hash: &str, timestamp: u64, transaction: &[ValidatedTransaction]) -> String {
        let transactions_data: String = transaction
        .iter()
        .map(|tx| format!("{}{}{}{}", tx.id, tx.sender, tx.receiver, tx.amount))
        .collect();

        let input = format!("{}{}{}{}", index, previous_hash, timestamp, transactions_data);
        let hash = blake3::hash(input.as_bytes());
        hash.to_hex().to_string()
    }
}