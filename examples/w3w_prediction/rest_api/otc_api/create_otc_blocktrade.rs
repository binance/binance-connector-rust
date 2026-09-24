use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::w3w_prediction::{
    W3WPredictionRestApi,
    rest_api::{CreateOtcBlocktradeParams, CreateOtcBlocktradeSideEnum},
};

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
    let params = CreateOtcBlocktradeParams::builder(
        "123".to_string(),
        "71321045679252212594626385532706912750332728571942532289631379312455583992563".to_string(),
        CreateOtcBlocktradeSideEnum::Buy,
        "600000000000000000000".to_string(),
        "1000000000000000000000".to_string(),
        "0.65".to_string(),
        1790000000,
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .create_otc_blocktrade(params)
        .await
        .context("create_otc_blocktrade request failed")?;

    info!(?response.rate_limits, "create_otc_blocktrade rate limits");
    let data = response.data().await?;
    info!(?data, "create_otc_blocktrade data");

    Ok(())
}
