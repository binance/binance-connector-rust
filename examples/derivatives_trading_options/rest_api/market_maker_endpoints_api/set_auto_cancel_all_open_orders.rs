use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_options::{
    DerivativesTradingOptionsRestApi, rest_api::SetAutoCancelAllOpenOrdersParams,
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

    // Create the DerivativesTradingOptions REST API client
    let rest_client = DerivativesTradingOptionsRestApi::production(rest_conf);

    // Setup the API parameters
    let params =
        SetAutoCancelAllOpenOrdersParams::builder("underlying_example".to_string(), 789).build()?;

    // Make the API call
    let response = rest_client
        .set_auto_cancel_all_open_orders(params)
        .await
        .context("set_auto_cancel_all_open_orders request failed")?;

    info!(?response.rate_limits, "set_auto_cancel_all_open_orders rate limits");
    let data = response.data().await?;
    info!(?data, "set_auto_cancel_all_open_orders data");

    Ok(())
}
