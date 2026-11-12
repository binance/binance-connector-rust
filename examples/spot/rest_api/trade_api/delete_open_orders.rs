use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::spot::{SpotRestApi, rest_api::DeleteOpenOrdersParams};

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

    // Create the Spot REST API client
    let rest_client = SpotRestApi::production(rest_conf);

    // Setup the API parameters
    let params = DeleteOpenOrdersParams::builder("BNBUSDT".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .delete_open_orders(params)
        .await
        .context("delete_open_orders request failed")?;

    info!(?response.rate_limits, "delete_open_orders rate limits");
    let data = response.data().await?;
    info!(?data, "delete_open_orders data");

    Ok(())
}
