use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::dual_investment::{
    DualInvestmentRestApi, rest_api::CheckDualInvestmentAccountsParams,
};
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

    // Create the DualInvestment REST API client
    let rest_client = DualInvestmentRestApi::production(rest_conf);

    // Setup the API parameters
    let params = CheckDualInvestmentAccountsParams::default();

    // Make the API call
    let response = rest_client
        .check_dual_investment_accounts(params)
        .await
        .context("check_dual_investment_accounts request failed")?;

    info!(?response.rate_limits, "check_dual_investment_accounts rate limits");
    let data = response.data().await?;
    info!(?data, "check_dual_investment_accounts data");

    Ok(())
}
