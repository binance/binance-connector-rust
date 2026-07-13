use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::vip_loan::{VIPLoanRestApi, rest_api::VipLoanFixedRateBorrowParams};

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

    // Create the VIPLoan REST API client
    let rest_client = VIPLoanRestApi::production(rest_conf);

    // Setup the API parameters
    let params = VipLoanFixedRateBorrowParams::builder(
        "1212:0.12:100;3434:0.13:50".to_string(),
        "BUSD".to_string(),
        30,
        12345678,
        "BNB,ETH,BTC".to_string(),
        "12345,67890,13579".to_string(),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .vip_loan_fixed_rate_borrow(params)
        .await
        .context("vip_loan_fixed_rate_borrow request failed")?;

    info!(?response.rate_limits, "vip_loan_fixed_rate_borrow rate limits");
    let data = response.data().await?;
    info!(?data, "vip_loan_fixed_rate_borrow data");

    Ok(())
}
