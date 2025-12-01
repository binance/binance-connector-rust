use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::margin_trading::{
    MarginTradingRestApi, rest_api::QueryCrossMarginAccountDetailsParams,
};

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

    // Create the MarginTrading REST API client
    let rest_client = MarginTradingRestApi::production(rest_conf);

    // Setup the API parameters
    let params = QueryCrossMarginAccountDetailsParams::default();

    // Make the API call
    let response = rest_client
        .query_cross_margin_account_details(params)
        .await
        .context("query_cross_margin_account_details request failed")?;

    info!(?response.rate_limits, "query_cross_margin_account_details rate limits");
    let data = response.data().await?;
    info!(?data, "query_cross_margin_account_details data");

    Ok(())
}
