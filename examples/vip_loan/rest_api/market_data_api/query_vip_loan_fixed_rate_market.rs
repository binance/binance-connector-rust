use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::vip_loan::{VIPLoanRestApi, rest_api::QueryVipLoanFixedRateMarketParams};

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
    let params = QueryVipLoanFixedRateMarketParams::builder("USDT".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .query_vip_loan_fixed_rate_market(params)
        .await
        .context("query_vip_loan_fixed_rate_market request failed")?;

    info!(?response.rate_limits, "query_vip_loan_fixed_rate_market rate limits");
    let data = response.data().await?;
    info!(?data, "query_vip_loan_fixed_rate_market data");

    Ok(())
}
