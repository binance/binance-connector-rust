use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi, rest_api::QueryInsuranceFundBalanceSnapshotParams,
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

    // Create the DerivativesTradingUsdsFutures REST API client
    let rest_client = DerivativesTradingUsdsFuturesRestApi::production(rest_conf);

    // Setup the API parameters
    let params = QueryInsuranceFundBalanceSnapshotParams::default();

    // Make the API call
    let response = rest_client
        .query_insurance_fund_balance_snapshot(params)
        .await
        .context("query_insurance_fund_balance_snapshot request failed")?;

    info!(?response.rate_limits, "query_insurance_fund_balance_snapshot rate limits");
    let data = response.data().await?;
    info!(?data, "query_insurance_fund_balance_snapshot data");

    Ok(())
}
