use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::w3w_prediction::{W3WPredictionRestApi, rest_api::QueryLastTradePriceParams};

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

    // Create the W3WPrediction REST API client
    let rest_client = W3WPredictionRestApi::production(rest_conf);

    // Setup the API parameters
    let params = QueryLastTradePriceParams::builder(5567895).build()?;

    // Make the API call
    let response = rest_client
        .query_last_trade_price(params)
        .await
        .context("query_last_trade_price request failed")?;

    info!(?response.rate_limits, "query_last_trade_price rate limits");
    let data = response.data().await?;
    info!(?data, "query_last_trade_price data");

    Ok(())
}
