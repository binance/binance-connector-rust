use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::vip_loan::{VIPLoanRestApi, rest_api::CheckVipLoanCollateralAccountParams};

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
    let params = CheckVipLoanCollateralAccountParams::default();

    // Make the API call
    let response = rest_client
        .check_vip_loan_collateral_account(params)
        .await
        .context("check_vip_loan_collateral_account request failed")?;

    info!(?response.rate_limits, "check_vip_loan_collateral_account rate limits");
    let data = response.data().await?;
    info!(?data, "check_vip_loan_collateral_account data");

    Ok(())
}
