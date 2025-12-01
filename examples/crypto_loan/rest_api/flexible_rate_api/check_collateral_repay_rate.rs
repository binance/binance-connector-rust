use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::crypto_loan::{CryptoLoanRestApi, rest_api::CheckCollateralRepayRateParams};
use binance_sdk::logger;

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

    // Create the CryptoLoan REST API client
    let rest_client = CryptoLoanRestApi::production(rest_conf);

    // Setup the API parameters
    let params = CheckCollateralRepayRateParams::builder(
        "loan_coin_example".to_string(),
        "collateral_coin_example".to_string(),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .check_collateral_repay_rate(params)
        .await
        .context("check_collateral_repay_rate request failed")?;

    info!(?response.rate_limits, "check_collateral_repay_rate rate limits");
    let data = response.data().await?;
    info!(?data, "check_collateral_repay_rate data");

    Ok(())
}
