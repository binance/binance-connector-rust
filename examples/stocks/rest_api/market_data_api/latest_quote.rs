use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::stocks::{StocksRestApi, rest_api::LatestQuoteParams};

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

    // Create the Stocks REST API client
    let rest_client = StocksRestApi::production(rest_conf);

    // Setup the API parameters
    let params = LatestQuoteParams::builder("AAPL".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .latest_quote(params)
        .await
        .context("latest_quote request failed")?;

    info!(?response.rate_limits, "latest_quote rate limits");
    let data = response.data().await?;
    info!(?data, "latest_quote data");

    Ok(())
}
