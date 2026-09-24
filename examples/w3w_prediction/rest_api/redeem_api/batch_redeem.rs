use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::w3w_prediction::{W3WPredictionRestApi, rest_api::BatchRedeemParams};

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
    let params = BatchRedeemParams::builder(
        "0x12e32db8817e292508c34111cbc4b23340df542c".to_string(),
        "5b5c1ec3be4e4416a5872b21c1ca5d20".to_string(),
        ["112233".to_string()].to_vec(),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .batch_redeem(params)
        .await
        .context("batch_redeem request failed")?;

    info!(?response.rate_limits, "batch_redeem rate limits");
    let data = response.data().await?;
    info!(?data, "batch_redeem data");

    Ok(())
}
