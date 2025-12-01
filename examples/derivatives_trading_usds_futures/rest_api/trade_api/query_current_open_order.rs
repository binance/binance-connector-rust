use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi, rest_api::QueryCurrentOpenOrderParams,
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

    // Create the DerivativesTradingUsdsFutures REST API client
    let rest_client = DerivativesTradingUsdsFuturesRestApi::production(rest_conf);

    // Setup the API parameters
    let params = QueryCurrentOpenOrderParams::builder("symbol_example".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .query_current_open_order(params)
        .await
        .context("query_current_open_order request failed")?;

    info!(?response.rate_limits, "query_current_open_order rate limits");
    let data = response.data().await?;
    info!(?data, "query_current_open_order data");

    Ok(())
}
