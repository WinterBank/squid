/// Calculate the mining reward based on the difficulty.
pub fn calculate_reward(difficulty: u64) -> u64 {
    (1.524287f64.powi(difficulty as i32)) as u64
}

static TOTAL_DISTRIBUTED: AtomicU64 = AtomicU64::new(0);

pub fn distribute_reward(miner_address: &str, reward: u64) {
    let total = TOTAL_DISTRIBUTED.fetch_add(reward, Ordering::SeqCst);
    if total + reward > MiningConfig::new().max_supply {
        panic!("Max supply exceeded");
    }
    println!("Distributed {} coins to miner {}", reward, miner_address);
}