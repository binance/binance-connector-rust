use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::stocks::{StocksRestApi, rest_api::TokenizedConvertHistoryParams};

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
    let params = TokenizedConvertHistoryParams::default();

    // Make the API call
    let response = rest_client
        .tokenized_convert_history(params)
        .await
        .context("tokenized_convert_history request failed")?;

    info!(?response.rate_limits, "tokenized_convert_history rate limits");
    let data = response.data().await?;
    info!(?data, "tokenized_convert_history data");

    Ok(())
}
