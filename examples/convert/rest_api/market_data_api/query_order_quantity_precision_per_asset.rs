use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::convert::{ConvertRestApi, rest_api::QueryOrderQuantityPrecisionPerAssetParams};
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
    let params = QueryOrderQuantityPrecisionPerAssetParams::default();

    // Make the API call
    let response = rest_client
        .query_order_quantity_precision_per_asset(params)
        .await
        .context("query_order_quantity_precision_per_asset request failed")?;

    info!(?response.rate_limits, "query_order_quantity_precision_per_asset rate limits");
    let data = response.data().await?;
    info!(?data, "query_order_quantity_precision_per_asset data");

    Ok(())
}
