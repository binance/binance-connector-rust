use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::margin_trading::{
    MarginTradingRestApi,
    rest_api::{MarginAccountNewOcoParams, MarginAccountNewOcoSideEnum},
};

#[tokio::main]
async fn main() -> Result<()> {
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
    let params = MarginAccountNewOcoParams::builder(
        "symbol_example".to_string(),
        MarginAccountNewOcoSideEnum::Buy,
        1.0,
        1.0,
        1.0,
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .margin_account_new_oco(params)
        .await
        .context("margin_account_new_oco request failed")?;

    info!(?response.rate_limits, "margin_account_new_oco rate limits");
    let data = response.data().await?;
    info!(?data, "margin_account_new_oco data");

    Ok(())
}
