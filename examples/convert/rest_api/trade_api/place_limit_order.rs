use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::convert::{ConvertRestApi, rest_api::PlaceLimitOrderParams};
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

    // Create the Convert REST API client
    let rest_client = ConvertRestApi::production(rest_conf);

    // Setup the API parameters
    let params = PlaceLimitOrderParams::builder(
        "base_asset_example".to_string(),
        "quote_asset_example".to_string(),
        dec!(1.0),
        "BUY".to_string(),
        "expired_type_example".to_string(),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .place_limit_order(params)
        .await
        .context("place_limit_order request failed")?;

    info!(?response.rate_limits, "place_limit_order rate limits");
    let data = response.data().await?;
    info!(?data, "place_limit_order data");

    Ok(())
}
