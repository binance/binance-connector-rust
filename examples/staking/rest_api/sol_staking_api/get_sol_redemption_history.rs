use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::staking::{StakingRestApi, rest_api::GetSolRedemptionHistoryParams};

#[tokio::main]
async fn main() -> Result<()> {
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
    let params = GetSolRedemptionHistoryParams::default();

    // Make the API call
    let response = rest_client
        .get_sol_redemption_history(params)
        .await
        .context("get_sol_redemption_history request failed")?;

    info!(?response.rate_limits, "get_sol_redemption_history rate limits");
    let data = response.data().await?;
    info!(?data, "get_sol_redemption_history data");

    Ok(())
}
