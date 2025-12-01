use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::algo::{AlgoRestApi, rest_api::CancelAlgoOrderSpotAlgoParams};
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

    // Create the Algo REST API client
    let rest_client = AlgoRestApi::production(rest_conf);

    // Setup the API parameters
    let params = CancelAlgoOrderSpotAlgoParams::builder(1).build()?;

    // Make the API call
    let response = rest_client
        .cancel_algo_order_spot_algo(params)
        .await
        .context("cancel_algo_order_spot_algo request failed")?;

    info!(?response.rate_limits, "cancel_algo_order_spot_algo rate limits");
    let data = response.data().await?;
    info!(?data, "cancel_algo_order_spot_algo data");

    Ok(())
}
