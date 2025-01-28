/// Mining configuration for Squid.
pub struct MiningConfig {
    pub difficulty: u64,
    pub reward_formula: fn(u64) -> u64,
    pub target_block_time: u64,
    pub max_supply: u64,
}

impl MiningConfig {
    /// Create a new `MiningConfig` with default values.
    pub fn new() -> Self {
        MiningConfig {
            difficulty: 1, // Initial difficulty
            reward_formula: |difficulty| (1.524287f64.powi(difficulty as i32)) as u64,
            target_block_time: 10, // 10 seconds
            max_supply: 2_305_843_009_213_693_951,
        }
    }
}