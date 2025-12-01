use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::wallet::{WalletRestApi, rest_api::OneClickArrivalDepositApplyParams};

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
    let params = OneClickArrivalDepositApplyParams::default();

    // Make the API call
    let response = rest_client
        .one_click_arrival_deposit_apply(params)
        .await
        .context("one_click_arrival_deposit_apply request failed")?;

    info!(?response.rate_limits, "one_click_arrival_deposit_apply rate limits");
    let data = response.data().await?;
    info!(?data, "one_click_arrival_deposit_apply data");

    Ok(())
}
