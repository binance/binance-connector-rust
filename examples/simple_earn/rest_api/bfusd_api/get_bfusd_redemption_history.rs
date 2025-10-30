use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::simple_earn::{SimpleEarnRestApi, rest_api::GetBfusdRedemptionHistoryParams};

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

    // Create the SimpleEarn REST API client
    let rest_client = SimpleEarnRestApi::production(rest_conf);

    // Setup the API parameters
    let params = GetBfusdRedemptionHistoryParams::default();

    // Make the API call
    let response = rest_client
        .get_bfusd_redemption_history(params)
        .await
        .context("get_bfusd_redemption_history request failed")?;

    info!(?response.rate_limits, "get_bfusd_redemption_history rate limits");
    let data = response.data().await?;
    info!(?data, "get_bfusd_redemption_history data");

    Ok(())
}
