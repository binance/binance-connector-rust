use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::vip_loan::{VIPLoanRestApi, rest_api::VipLoanBorrowParams};

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
    let params = VipLoanBorrowParams::builder(
        1,
        "loan_coin_example".to_string(),
        dec!(1.0),
        "1".to_string(),
        "collateral_coin_example".to_string(),
        true,
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .vip_loan_borrow(params)
        .await
        .context("vip_loan_borrow request failed")?;

    info!(?response.rate_limits, "vip_loan_borrow rate limits");
    let data = response.data().await?;
    info!(?data, "vip_loan_borrow data");

    Ok(())
}
