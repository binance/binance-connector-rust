use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::stocks::{StocksRestApi, rest_api::TokenizedMintParams};

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
    let params = TokenizedMintParams::builder("AAPL".to_string(), "1".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .tokenized_mint(params)
        .await
        .context("tokenized_mint request failed")?;

    info!(?response.rate_limits, "tokenized_mint rate limits");
    let data = response.data().await?;
    info!(?data, "tokenized_mint data");

    Ok(())
}
