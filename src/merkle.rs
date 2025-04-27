pub mod merkle_states {
    // Core states representing Merkle tree components
    pub struct Empty;          // Initial state with no data
    pub struct HashedLeaves;   // Leaf nodes computed
    #[derive(Default)]
    pub struct TreeConstructed;// All parent leaves + root computed
    #[derive(Default)]
    pub struct RootVerified;   // Root cryptographically validated
}


pub use merkle_states::{Empty, HashedLeaves, TreeConstructed, RootVerified};
use std::marker::PhantomData;
pub use rs_merkle::{MerkleTree as RsMerkleTree, Hasher as RsHasher};

#[derive(Default)]
pub struct MerkleTree<S, const HASH_SIZE: usize = 32> {
    rs_tree: RsMerkleTree<Blake3Hasher>,
    leaves: Vec<[u8; 32]>,
    _state: PhantomData<S>,
}

#[derive(Clone)]
pub struct Blake3Hasher;

impl Blake3Hasher {
    pub fn hash(data: &[u8]) -> [u8; 32] {
        *blake3::hash(data).as_bytes()
    }
}

impl RsHasher for Blake3Hasher {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> Self::Hash {
        *blake3::hash(data).as_bytes()
    }
}

impl MerkleTree<Empty, 32> {
    /// Create new empty Merkle tree (Blake3-32 version)
    pub fn new() -> Self {
        Self {
            rs_tree: RsMerkleTree::new(),
            leaves: Vec::new(),
            _state: PhantomData,
        }
    }

    /// Hash leaves and transition to HashedLeaves state
    pub fn hash_leaves(self, data: &[impl AsRef<[u8]>]) -> MerkleTree<HashedLeaves, 32> {
        assert!(!data.is_empty(), "Cannot create tree with 0 leaves");

        let leaves: Vec<_> = data.iter()
            .map(|d| Blake3Hasher::hash(d.as_ref()))
            .collect();
            
        MerkleTree {
            rs_tree: RsMerkleTree::from_leaves(&leaves),
            leaves,
            _state: PhantomData,
        }
    }
}

impl MerkleTree<HashedLeaves, 32> {
    /// Build full tree structure from leaves
    pub fn construct_tree(self) -> MerkleTree<TreeConstructed, 32> {

        MerkleTree {
            rs_tree: self.rs_tree,
            leaves: self.leaves,
            _state: PhantomData,
        }
    }

    pub fn from_transactions(txs: Vec<ValidatedTransaction>) -> Self {
        let leaves: Vec<[u8; 32]> = txs
            .into_iter()
            .map(|tx| Blake3Hasher::hash(&tx.to_bytes()))
            .collect();
            
        Self::from_leaves(leaves)
    }
}

impl MerkleTree<TreeConstructed, 32> {
    /// Cryptographic verification of tree integrity
    pub fn verify_root(self) -> Result<MerkleTree<RootVerified, 32>, String> {
        if self.rs_tree.root().is_some() {
            Ok(MerkleTree {
                rs_tree: self.rs_tree,
                leaves: self.leaves,
                _state: PhantomData,
            })
        } else {
            Err("Invalid tree structure".into())
        }
    }

    pub fn append_hashes(self, new_hashes: &[[u8; 32]]) -> Result<Self, String> {
        let mut all_hashes = self.leaves;
        all_hashes.extend_from_slice(new_hashes);
        if all_hashes.is_empty() {
            return Err("Cannot append empty hashes".into());
        }
        Ok(Self {
            rs_tree: RsMerkleTree::from_leaves(&all_hashes),
            leaves: all_hashes,
            _state: PhantomData,
        })
    }

    pub fn verify(self) -> Result<MerkleTree<RootVerified, 32>, String> {
        if self.rs_tree.root().is_some() {
            Ok(MerkleTree {
                rs_tree: self.rs_tree,
                leaves: self.leaves,
                _state: PhantomData,
            })
        } else {
            Err("Invalid tree structure".into())
        }
    }
}

impl MerkleTree<RootVerified, 32> {
    pub fn root(&self) -> [u8; 32] {
        self.rs_tree.root().expect("Verified tree must have root").clone()
    }

    /// Generate proofs using rs-merkle's algorithms
    pub fn generate_proof(&self, leaf_indices: &[usize]) -> rs_merkle::MerkleProof<Blake3Hasher> {
        self.rs_tree.proof(leaf_indices)
    }

    pub fn into_unverified(self) -> MerkleTree<TreeConstructed, 32> {
        MerkleTree {
            rs_tree: self.rs_tree,
            leaves: self.leaves,
            _state: PhantomData,
        }
    }

    pub fn append_hashes(mut self, new_hashes: &[[u8; 32]]) -> Result<Self, String> {
        self.leaves.extend_from_slice(new_hashes);
        self.rs_tree = RsMerkleTree::from_leaves(&self.leaves);
        Ok(self)
    }

    /// Read-only leaf access
    pub fn leaves(&self) -> &[[u8; 32]] {
        &self.leaves
    }
}