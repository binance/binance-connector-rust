use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::staking::{StakingRestApi, rest_api::GetSoftStakingRewardsHistoryParams};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise logging
    logger::init();

    // Load credentials from env
    let api_key = env::var("API_KEY").context("API_KEY must be set")?;
    let api_secret = env::var("API_SECRET").context("API_SECRET must be set")?;

    // Build REST config
    let rest_conf = ConfigurationRestApi::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()?;

    // Create the Staking REST API client
    let rest_client = StakingRestApi::production(rest_conf);

    // Setup the API parameters
    let params = GetSoftStakingRewardsHistoryParams::default();

    // Make the API call
    let response = rest_client
        .get_soft_staking_rewards_history(params)
        .await
        .context("get_soft_staking_rewards_history request failed")?;

    info!(?response.rate_limits, "get_soft_staking_rewards_history rate limits");
    let data = response.data().await?;
    info!(?data, "get_soft_staking_rewards_history data");

    Ok(())
}
