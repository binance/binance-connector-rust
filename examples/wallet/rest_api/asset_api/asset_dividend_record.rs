use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::wallet::{WalletRestApi, rest_api::AssetDividendRecordParams};

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
    let params = AssetDividendRecordParams::default();

    // Make the API call
    let response = rest_client
        .asset_dividend_record(params)
        .await
        .context("asset_dividend_record request failed")?;

    info!(?response.rate_limits, "asset_dividend_record rate limits");
    let data = response.data().await?;
    info!(?data, "asset_dividend_record data");

    Ok(())
}
