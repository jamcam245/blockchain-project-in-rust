use blake3;
use std::marker::PhantomData;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::transaction::ValidatedTransaction;

pub struct Uninitialized;
pub struct Initialized;

pub struct Verified {pub last_hash: [u8; 32], pub last_timestamp: u64}

pub struct Poh<S> {pub entries: Vec<PohEntry>, state: S, _marker: PhantomData<*mut ()>}

#[derive(Clone)]
pub struct PohEntry {pub timestamp: u64, pub poh_hash: [u8; 32], pub transactions: Vec<ValidatedTransaction>}

impl<S> Poh<S> {
    #[inline(always)]
    fn calculate_hash(prev_hash: &[u8; 32], timestamp: u64, transactions: &[ValidatedTransaction]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(prev_hash);
        hasher.update(&timestamp.to_le_bytes());
        for tx in transactions {
            hasher.update(&tx.to_bytes());
        }
        *hasher.finalize().as_bytes()
    }
}

impl Poh<Uninitialized> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            state: Uninitialized,
            _marker: PhantomData,
        }
    }

    pub fn initialize(self) -> Poh<Initialized> {
        let genesis_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

        let genesis_hash = *blake3::hash(b"genesis").as_bytes();

        Poh {
            entries: vec![PohEntry {
                timestamp: genesis_timestamp,
                poh_hash: genesis_hash,
                transactions: Vec::new(),
            }],
            state: Initialized,
            _marker: PhantomData,
        }
    }
}

impl Poh<Initialized> {
    pub fn verify_genesis(self) -> Result<Poh<Verified>, &'static str> {

        let mut entries = self.entries;

        if entries.is_empty() {
            return Err("Genesis block missing");
        }

        let genesis = entries.remove(0);

        let expected_genesis_hash = *blake3::hash(b"genesis").as_bytes();
        
        if genesis.poh_hash != expected_genesis_hash {
            return Err("Invalid genesis hash");
        }

        let genesis_hash = genesis.poh_hash;
        let genesis_timestamp = genesis.timestamp;

        let mut verified_genesis = Vec::with_capacity(entries.len() + 1);
        verified_genesis.push(genesis);
        verified_genesis.extend(entries);

        Ok(Poh {
            entries: verified_genesis,
            state: Verified {
                last_hash: genesis_hash,
                last_timestamp: genesis_timestamp,
            },
            _marker: PhantomData,
        })
    }
}

impl Poh<Verified> {
    pub fn append_entry(&mut self, transaction: Option<ValidatedTransaction>) -> [u8; 32] {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.state.last_hash);
        hasher.update(&timestamp.to_le_bytes());

        if let Some(transaction) = &transaction {
            hasher.update(&transaction.to_bytes());
        }

        let poh_hash = *hasher.finalize().as_bytes();

        self.entries.push(PohEntry { timestamp, poh_hash, transactions: transaction.into_iter().collect()});

        self.state.last_hash = poh_hash;
        self.state.last_timestamp = timestamp;
        poh_hash
    }

    pub fn append_entries(&mut self, transactions: Vec<ValidatedTransaction>) -> Vec<[u8; 32]> {
        assert!(!transactions.is_empty(), "Transactions required");
    
        let base_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let count = transactions.len();
        
        let mut hashes = Vec::with_capacity(count);
        let mut current_hash = self.state.last_hash;
        let mut hasher = blake3::Hasher::new();
        
        self.entries.extend(
            transactions.into_iter().zip(0..count)
                .map(|(tx, offset)| {
                    let timestamp = base_time + offset as u64;
                    
                    hasher.reset();
                    hasher.update(&current_hash);
                    hasher.update(&timestamp.to_le_bytes());
                    hasher.update(&tx.to_bytes());
                    
                    current_hash = *hasher.finalize().as_bytes();
                    hashes.push(current_hash);
                    
                    PohEntry {
                        timestamp,
                        poh_hash: current_hash,
                        transactions: vec![tx],
                    }
                })
        );
    
        self.state.last_hash = current_hash;
        self.state.last_timestamp = base_time + (count as u64 - 1);
        hashes
    }

    pub fn verify_entries(&self) -> bool {
        if self.entries.is_empty() {
            return false;
        }
        
        let mut prev_hash = self.entries[0].poh_hash;
        for entry in &self.entries[1..] {
            let computed_hash = Self::calculate_hash(&prev_hash, entry.timestamp, &entry.transactions);
            if computed_hash != entry.poh_hash {
                return false;
            }
            prev_hash = entry.poh_hash;
        }
        true
    }
}