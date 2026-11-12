use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_options::{
    DerivativesTradingOptionsRestApi, rest_api::RecentBlockTradesListParams,
};
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

    // Create the DerivativesTradingOptions REST API client
    let rest_client = DerivativesTradingOptionsRestApi::production(rest_conf);

    // Setup the API parameters
    let params = RecentBlockTradesListParams::default();

    // Make the API call
    let response = rest_client
        .recent_block_trades_list(params)
        .await
        .context("recent_block_trades_list request failed")?;

    info!(?response.rate_limits, "recent_block_trades_list rate limits");
    let data = response.data().await?;
    info!(?data, "recent_block_trades_list data");

    Ok(())
}
