mod block;
mod poh;
mod transaction;

use transaction::{Transaction};
use block::{PendingBlock};
use poh::{UninitializedPoh};

fn main() {
    // Create and validate a transaction
    let mut tx = Transaction::new(
        1,
        "Alice".to_string(),
        "Bob".to_string(),
        100.0
    );

    tx.sign("valid_signature".to_string());
    
    let validated_tx = match tx.validate() {
        Ok(vtx) => vtx,
        Err(e) => {
            println!("Transaction validation failed: {}", e);
            return;
        }
    };

    // Create a new Uninitialized PoH instance
    // Initialize the PoH with a genesis hash and parameters
    // Note: In a real-world scenario, the genesis hash would be a cryptographic hash of the previous block
    // and the parameters would be derived from the blockchain's consensus rules.
    // For this example, we are using a dummy genesis hash and arbitrary parameters.
    let uninitialized_poh = UninitializedPoh::new();

    let Ok(verified_poh) = uninitialized_poh
        .initialize("genesis_hash", 10, 0, 1)
        .and_then(|poh| poh.verify("genesis_hash")) else {
            eprintln!("PoH failed");
            return;
        };    
    

    // Create and finalize a block with the validated transaction
    let mut pending_block = PendingBlock::new("0".to_string(), 10);
    pending_block.add_transaction(validated_tx);

    // Get a PoH reference for the block
    let poh_reference = verified_poh.get_sequence()[0].clone();
    
    // Finalize the block
    let committed_block = pending_block.finalize(1, poh_reference);

    // Verify the committed block using the verified PoH
    if committed_block.verify(&verified_poh) {
        println!("Block successfully created and verified!");
        println!("Block: {:?}", committed_block);
    } else {
        println!("Block verification failed!");
    }
}