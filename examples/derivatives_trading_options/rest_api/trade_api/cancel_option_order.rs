use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_options::{
    DerivativesTradingOptionsRestApi, rest_api::CancelOptionOrderParams,
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
    let params = CancelOptionOrderParams::builder("symbol_example".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .cancel_option_order(params)
        .await
        .context("cancel_option_order request failed")?;

    info!(?response.rate_limits, "cancel_option_order rate limits");
    let data = response.data().await?;
    info!(?data, "cancel_option_order data");

    Ok(())
}
