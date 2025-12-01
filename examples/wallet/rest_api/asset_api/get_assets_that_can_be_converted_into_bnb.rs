use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::wallet::{WalletRestApi, rest_api::GetAssetsThatCanBeConvertedIntoBnbParams};

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

    // Create the Wallet REST API client
    let rest_client = WalletRestApi::production(rest_conf);

    // Setup the API parameters
    let params = GetAssetsThatCanBeConvertedIntoBnbParams::default();

    // Make the API call
    let response = rest_client
        .get_assets_that_can_be_converted_into_bnb(params)
        .await
        .context("get_assets_that_can_be_converted_into_bnb request failed")?;

    info!(?response.rate_limits, "get_assets_that_can_be_converted_into_bnb rate limits");
    let data = response.data().await?;
    info!(?data, "get_assets_that_can_be_converted_into_bnb data");

    Ok(())
}
