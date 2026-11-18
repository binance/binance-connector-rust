use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::stock_options::{StockOptionsRestApi, rest_api::OptionTradeHistoryParams};

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

    // Create the StockOptions REST API client
    let rest_client = StockOptionsRestApi::production(rest_conf);

    // Setup the API parameters
    let params = OptionTradeHistoryParams::builder(1790100000000, 1790200000000).build()?;

    // Make the API call
    let response = rest_client
        .option_trade_history(params)
        .await
        .context("option_trade_history request failed")?;

    info!(?response.rate_limits, "option_trade_history rate limits");
    let data = response.data().await?;
    info!(?data, "option_trade_history data");

    Ok(())
}
