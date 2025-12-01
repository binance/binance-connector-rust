use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::convert::{ConvertRestApi, rest_api::GetConvertTradeHistoryParams};
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

    // Create the Convert REST API client
    let rest_client = ConvertRestApi::production(rest_conf);

    // Setup the API parameters
    let params = GetConvertTradeHistoryParams::builder(1623319461670, 1641782889000).build()?;

    // Make the API call
    let response = rest_client
        .get_convert_trade_history(params)
        .await
        .context("get_convert_trade_history request failed")?;

    info!(?response.rate_limits, "get_convert_trade_history rate limits");
    let data = response.data().await?;
    info!(?data, "get_convert_trade_history data");

    Ok(())
}
