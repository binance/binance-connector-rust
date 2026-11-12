use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::margin_trading::{
    MarginTradingRestApi, rest_api::QueryMarginAccountsOpenOcoParams,
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
    let params = QueryMarginAccountsOpenOcoParams::default();

    // Make the API call
    let response = rest_client
        .query_margin_accounts_open_oco(params)
        .await
        .context("query_margin_accounts_open_oco request failed")?;

    info!(?response.rate_limits, "query_margin_accounts_open_oco rate limits");
    let data = response.data().await?;
    info!(?data, "query_margin_accounts_open_oco data");

    Ok(())
}
