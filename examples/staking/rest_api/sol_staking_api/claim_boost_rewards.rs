use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::staking::{StakingRestApi, rest_api::ClaimBoostRewardsParams};

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
    let params = ClaimBoostRewardsParams::default();

    // Make the API call
    let response = rest_client
        .claim_boost_rewards(params)
        .await
        .context("claim_boost_rewards request failed")?;

    info!(?response.rate_limits, "claim_boost_rewards rate limits");
    let data = response.data().await?;
    info!(?data, "claim_boost_rewards data");

    Ok(())
}
