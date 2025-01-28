use poem_openapi::{param::{Path, Query}, OpenApi, payload::Json};
use poem::Result;
use serde_json::json;
use crate::mining::mining_config::MiningConfig;
use crate::mining::validation::{validate_hash, generate_argon2d_hash};
use crate::mining::reward::{calculate_reward, distribute_reward};
use crate::mining::integration::issue_reward;

/// Represents a mining solution submitted by a miner.
#[derive(poem_openapi::Object, serde::Deserialize, serde::Serialize)]
pub struct MiningSolution {
    pub account_key: String,
    pub nonce: u64,
    pub hash: String,
    pub salt: String,
}

/// API for mining-related endpoints.
pub struct MiningApi;

#[OpenApi]
impl MiningApi {
    /// Endpoint to fetch current mining difficulty.
    #[oai(
        path = "/mining/difficulty",
        method = "get",
        operation_id = "get_difficulty",
        tag = "Mining"
    )]
    async fn get_difficulty(&self) -> Result<Json<serde_json::Value>> {
        let config = MiningConfig::new();
        Ok(Json(json!({
            "difficulty": config.difficulty,
            "target_block_time": config.target_block_time,
        })))
    }

    /// Endpoint to submit a mining solution.
    #[oai(
        path = "/mining/submit",
        method = "post",
        operation_id = "submit_solution",
        tag = "Mining"
    )]
    async fn submit_solution(&self, solution: Json<MiningSolution>) -> Result<Json<serde_json::Value>> {
        let solution = solution.into_inner();

        // Validate the mining solution.
        if !validate_hash(&solution.hash, &solution.salt, MiningConfig::new().difficulty) {
            return Ok(Json(json!({
                "status": "error",
                "message": "Invalid mining solution",
            })));
        }

        // Calculate the reward.
        let reward = calculate_reward(MiningConfig::new().difficulty);

        // Distribute the reward to the miner's account.
        distribute_reward(&solution.account_key, reward);

        // Issue the reward via blockchain logic.
        issue_reward(&solution.account_key, reward);

        Ok(Json(json!({
            "status": "success",
            "reward": reward,
        })))
    }
}