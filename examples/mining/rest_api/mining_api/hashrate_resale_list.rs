use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::mining::{MiningRestApi, rest_api::HashrateResaleListParams};

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

    // Create the Mining REST API client
    let rest_client = MiningRestApi::production(rest_conf);

    // Setup the API parameters
    let params = HashrateResaleListParams::default();

    // Make the API call
    let response = rest_client
        .hashrate_resale_list(params)
        .await
        .context("hashrate_resale_list request failed")?;

    info!(?response.rate_limits, "hashrate_resale_list rate limits");
    let data = response.data().await?;
    info!(?data, "hashrate_resale_list data");

    Ok(())
}
