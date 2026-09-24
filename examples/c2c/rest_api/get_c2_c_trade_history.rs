use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::c2c::{C2CRestApi, rest_api::GetC2CTradeHistoryParams};
use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;

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

    // Create the C2C REST API client
    let rest_client = C2CRestApi::production(rest_conf);

    // Setup the API parameters
    let params = GetC2CTradeHistoryParams::default();

    // Make the API call
    let response = rest_client
        .get_c2_c_trade_history(params)
        .await
        .context("get_c2_c_trade_history request failed")?;

    info!(?response.rate_limits, "get_c2_c_trade_history rate limits");
    let data = response.data().await?;
    info!(?data, "get_c2_c_trade_history data");

    Ok(())
}
