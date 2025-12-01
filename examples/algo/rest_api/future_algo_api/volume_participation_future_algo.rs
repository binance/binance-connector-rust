use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::algo::{AlgoRestApi, rest_api::VolumeParticipationFutureAlgoParams};
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
    let params = VolumeParticipationFutureAlgoParams::builder(
        "BTCUSDT".to_string(),
        "BUY".to_string(),
        dec!(1.0),
        "LOW".to_string(),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .volume_participation_future_algo(params)
        .await
        .context("volume_participation_future_algo request failed")?;

    info!(?response.rate_limits, "volume_participation_future_algo rate limits");
    let data = response.data().await?;
    info!(?data, "volume_participation_future_algo data");

    Ok(())
}
