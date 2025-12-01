use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::wallet::{WalletRestApi, rest_api::DepositHistoryV2Params};

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
    let params = DepositHistoryV2Params::default();

    // Make the API call
    let response = rest_client
        .deposit_history_v2(params)
        .await
        .context("deposit_history_v2 request failed")?;

    info!(?response.rate_limits, "deposit_history_v2 rate limits");
    let data = response.data().await?;
    info!(?data, "deposit_history_v2 data");

    Ok(())
}
