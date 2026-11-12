use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::sub_account::{SubAccountRestApi, rest_api::SubAccountTransferHistoryParams};

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

    // Create the SubAccount REST API client
    let rest_client = SubAccountRestApi::production(rest_conf);

    // Setup the API parameters
    let params = SubAccountTransferHistoryParams::default();

    // Make the API call
    let response = rest_client
        .sub_account_transfer_history(params)
        .await
        .context("sub_account_transfer_history request failed")?;

    info!(?response.rate_limits, "sub_account_transfer_history rate limits");
    let data = response.data().await?;
    info!(?data, "sub_account_transfer_history data");

    Ok(())
}
