use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::wallet::{WalletRestApi, rest_api::EnableFastWithdrawSwitchParams};

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
    let params = EnableFastWithdrawSwitchParams::default();

    // Make the API call
    let response = rest_client
        .enable_fast_withdraw_switch(params)
        .await
        .context("enable_fast_withdraw_switch request failed")?;

    info!(?response.rate_limits, "enable_fast_withdraw_switch rate limits");
    let data = response.data().await?;
    info!(?data, "enable_fast_withdraw_switch data");

    Ok(())
}
