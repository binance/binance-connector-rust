use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::w3w_prediction::{
    W3WPredictionRestApi,
    rest_api::{
        CreateOutboundTransferAccountTypeEnum, CreateOutboundTransferParams,
        CreateOutboundTransferSourceBizEnum,
    },
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
    let params = CreateOutboundTransferParams::builder(
        "5b5c1ec3be4e4416a5872b21c1ca5d20".to_string(),
        "0x12e32db8817e292508c34111cbc4b23340df542c".to_string(),
        "1000000000000000000".to_string(),
        CreateOutboundTransferAccountTypeEnum::Spot,
        CreateOutboundTransferSourceBizEnum::UserTransfer,
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .create_outbound_transfer(params)
        .await
        .context("create_outbound_transfer request failed")?;

    info!(?response.rate_limits, "create_outbound_transfer rate limits");
    let data = response.data().await?;
    info!(?data, "create_outbound_transfer data");

    Ok(())
}
