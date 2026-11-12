use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::convert::{ConvertRestApi, rest_api::QueryLimitOpenOrdersParams};
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
    let params = QueryLimitOpenOrdersParams::default();

    // Make the API call
    let response = rest_client
        .query_limit_open_orders(params)
        .await
        .context("query_limit_open_orders request failed")?;

    info!(?response.rate_limits, "query_limit_open_orders rate limits");
    let data = response.data().await?;
    info!(?data, "query_limit_open_orders data");

    Ok(())
}
