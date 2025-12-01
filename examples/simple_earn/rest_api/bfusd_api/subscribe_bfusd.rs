use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::simple_earn::{SimpleEarnRestApi, rest_api::SubscribeBfusdParams};

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

    // Create the SimpleEarn REST API client
    let rest_client = SimpleEarnRestApi::production(rest_conf);

    // Setup the API parameters
    let params = SubscribeBfusdParams::builder("asset_example".to_string(), dec!(1.0)).build()?;

    // Make the API call
    let response = rest_client
        .subscribe_bfusd(params)
        .await
        .context("subscribe_bfusd request failed")?;

    info!(?response.rate_limits, "subscribe_bfusd rate limits");
    let data = response.data().await?;
    info!(?data, "subscribe_bfusd data");

    Ok(())
}
