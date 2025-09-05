use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::simple_earn::{SimpleEarnRestApi, rest_api::RedeemFlexibleProductParams};

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
    let params = RedeemFlexibleProductParams::builder("1".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .redeem_flexible_product(params)
        .await
        .context("redeem_flexible_product request failed")?;

    info!(?response.rate_limits, "redeem_flexible_product rate limits");
    let data = response.data().await?;
    info!(?data, "redeem_flexible_product data");

    Ok(())
}
