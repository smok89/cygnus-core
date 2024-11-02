use super::blockchain::Blockchain;
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

// `Block`, A struct that represents a block in a Blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    // The index in which the current block is stored.
    pub index: u64,
    // The time the current block is created.
    pub timestamp: u64,
    // The block's proof of work.
    pub proof_of_work: u64,
    // The previous block hash.
    pub previous_hash: String,
    // The current block hash.
    pub hash: String,
    // The data contained in the block.
    pub data: String
}
impl Block {
    // Calculate block hash.
    pub fn calculate_hash(&self) -> String {
        let mut block_clone = self.clone();
        block_clone.hash = String::default();
        let serialized_block_data = serde_json::to_string(&block_clone).unwrap();
        // Calculate and return SHA-256 hash value.
        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // Create a new block.
    pub fn new (
        index: u64,
        previous_hash: String,
        data: String,
    ) -> Self {
        // Current block to be created.
        let mut block = Block {
            index,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash,
            hash: String::default(),
            data,
        };
        block.hash = block.calculate_hash();
        block
    }

    // Proof-of-work algorithm (mining).
    pub fn mine (&mut self, blockchain: &Blockchain) {
        loop {
            if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)) {
                self.proof_of_work += 1;
                self.hash = self.calculate_hash();
            } else {
                break
            }
        }
    }
}
