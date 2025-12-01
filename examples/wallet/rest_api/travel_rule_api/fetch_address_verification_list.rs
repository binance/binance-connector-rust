use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::wallet::{WalletRestApi, rest_api::FetchAddressVerificationListParams};

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
    let params = FetchAddressVerificationListParams::default();

    // Make the API call
    let response = rest_client
        .fetch_address_verification_list(params)
        .await
        .context("fetch_address_verification_list request failed")?;

    info!(?response.rate_limits, "fetch_address_verification_list rate limits");
    let data = response.data().await?;
    info!(?data, "fetch_address_verification_list data");

    Ok(())
}
