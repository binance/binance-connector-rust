use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::vip_loan::{VIPLoanRestApi, rest_api::VipLoanRepayParams};

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
    let params = VipLoanRepayParams::builder(1, dec!(1.0)).build()?;

    // Make the API call
    let response = rest_client
        .vip_loan_repay(params)
        .await
        .context("vip_loan_repay request failed")?;

    info!(?response.rate_limits, "vip_loan_repay rate limits");
    let data = response.data().await?;
    info!(?data, "vip_loan_repay data");

    Ok(())
}
