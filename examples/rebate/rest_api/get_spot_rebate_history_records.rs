use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::rebate::{RebateRestApi, rest_api::GetSpotRebateHistoryRecordsParams};

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

    // Create the Rebate REST API client
    let rest_client = RebateRestApi::production(rest_conf);

    // Setup the API parameters
    let params = GetSpotRebateHistoryRecordsParams::default();

    // Make the API call
    let response = rest_client
        .get_spot_rebate_history_records(params)
        .await
        .context("get_spot_rebate_history_records request failed")?;

    info!(?response.rate_limits, "get_spot_rebate_history_records rate limits");
    let data = response.data().await?;
    info!(?data, "get_spot_rebate_history_records data");

    Ok(())
}
