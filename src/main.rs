mod poh;
mod transaction;

use poh::{Poh, Uninitialized};
use crate::transaction::ValidatedTransaction;
use rs_merkle::{MerkleTree, Hasher};
use blake3;

#[derive(Clone)]
struct Blake3Hasher;

impl Hasher for Blake3Hasher {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        *blake3::hash(data).as_bytes()
    }
}

fn create_transactions(count: usize) -> Vec<ValidatedTransaction> {
    (0..count).map(|i| {
        let mut tx = transaction::Transaction::new(
            i as u32,
            format!("sender_{}", i),
            format!("receiver_{}", i),
            (i * 10) as f64
        );
        tx.sign(format!("signature_{}", i));
        tx.validate().expect("Transaction validation failed")
    }).collect()
}

fn main() {
    let poh = Poh::<Uninitialized>::new()
        .initialize();
    
    let mut verified_poh = poh.verify_genesis()
        .expect("PoH verification failed: Invalid genesis");
    println!("Genesis PoH sequence verified ✓");

    /*
    verified_poh.append_entry();
    verified_poh.append_entry();
    println!("Appended 2 new PoH entries");
    */

    let batch_size = 3;
    let transactions = create_transactions(batch_size);
    let hashes = verified_poh.append_entries(transactions);

    println!("Appended {} new PoH entries", batch_size);
    
    println!("Generated {} hashes:", hashes.len());
    for (i, hash) in hashes.iter().enumerate() {
        println!("{}: {:?}", i, hash);
    }

    assert!(verified_poh.verify_entries(), "Entry verification failed");
    println!("All entries verified ✓");

    let leaves: Vec<[u8; 32]> = verified_poh.entries
        .iter()
        .map(|e| e.poh_hash)
        .collect();

    let merkle_tree = MerkleTree::<Blake3Hasher>::from_leaves(&leaves);
    let root = merkle_tree.root().unwrap();

    for (i, entry) in verified_poh.entries.iter().enumerate() {
        let proof = merkle_tree.proof(&[i]);
        assert!(
            proof.verify(root, &[i], &[entry.poh_hash], leaves.len()),
            "Merkle verification failed for entry {}",
            i
        );
        println!("Entry {} verified in Merkle tree ✓", i);
    }

    println!("Final Merkle root: {:x?}", root);
    println!("All PoH hashes successfully stored in Merkle tree!");
}