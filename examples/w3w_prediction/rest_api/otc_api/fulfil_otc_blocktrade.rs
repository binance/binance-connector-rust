use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::w3w_prediction::{W3WPredictionRestApi, rest_api::FulfilOtcBlocktradeParams};

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

    // Create the W3WPrediction REST API client
    let rest_client = W3WPredictionRestApi::production(rest_conf);

    // Setup the API parameters
    let params = FulfilOtcBlocktradeParams::builder(
        "26080500000001234567".to_string(),
        "a1b2c3d4-e5f6-7890-abcd-ef1234567890".to_string(),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .fulfil_otc_blocktrade(params)
        .await
        .context("fulfil_otc_blocktrade request failed")?;

    info!(?response.rate_limits, "fulfil_otc_blocktrade rate limits");
    let data = response.data().await?;
    info!(?data, "fulfil_otc_blocktrade data");

    Ok(())
}
