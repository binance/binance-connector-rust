use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::alpha::AlphaRestApi;
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

    // Create the Alpha REST API client
    let rest_client = AlphaRestApi::production(rest_conf);

    // Make the API call
    let response = rest_client
        .get_exchange_info()
        .await
        .context("get_exchange_info request failed")?;

    info!(?response.rate_limits, "get_exchange_info rate limits");
    let data = response.data().await?;
    info!(?data, "get_exchange_info data");

    Ok(())
}
