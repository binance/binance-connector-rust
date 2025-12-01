use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_options::{
    DerivativesTradingOptionsRestApi, rest_api::CancelMultipleOptionOrdersParams,
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
    let params = CancelMultipleOptionOrdersParams::builder("symbol_example".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .cancel_multiple_option_orders(params)
        .await
        .context("cancel_multiple_option_orders request failed")?;

    info!(?response.rate_limits, "cancel_multiple_option_orders rate limits");
    let data = response.data().await?;
    info!(?data, "cancel_multiple_option_orders data");

    Ok(())
}
