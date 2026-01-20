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
        .token_list()
        .await
        .context("token_list request failed")?;

    info!(?response.rate_limits, "token_list rate limits");
    let data = response.data().await?;
    info!(?data, "token_list data");

    Ok(())
}
