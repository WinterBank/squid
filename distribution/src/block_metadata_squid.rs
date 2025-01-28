use aptos_types::block_metadata::BlockMetadata;
use aptos_types::account_address::AccountAddress;
use aptos_crypto::HashValue;
use serde::{Deserialize, Serialize};

/// Additional mining-related metadata.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MiningMetadata {
    pub solution_hash: HashValue,       // Verified hash of the solution
    pub miner_address: AccountAddress, // Address of the miner who submitted the solution
    pub reward: u64,                   // Reward issued for the valid solution
}

/// Extension trait for BlockMetadata to handle mining-related logic.
pub trait BlockMetadataMiningExtension {
    fn add_mining_metadata(&mut self, metadata: MiningMetadata);
    fn get_mining_metadata(&self) -> Option<&MiningMetadata>;
}

impl BlockMetadataMiningExtension for BlockMetadata {
    fn add_mining_metadata(&mut self, metadata: MiningMetadata) {
        // Logic to attach mining metadata to the block.
        // Ensure no overwriting of existing metadata.
        if self.mining_metadata().is_none() {
            self.mining_metadata = Some(metadata);
        } else {
            panic!("Mining metadata already exists for this block.");
        }
    }

    fn get_mining_metadata(&self) -> Option<&MiningMetadata> {
        self.mining_metadata.as_ref()
    }
}

impl BlockMetadata {
    pub fn mining_metadata(&self) -> Option<&MiningMetadata> {
        self.mining_metadata.as_ref()
    }
}
