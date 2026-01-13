use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_portfolio_margin_pro::{
    DerivativesTradingPortfolioMarginProRestApi, rest_api::SwitchDeltaModeParams,
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

    // Create the DerivativesTradingPortfolioMarginPro REST API client
    let rest_client = DerivativesTradingPortfolioMarginProRestApi::production(rest_conf);

    // Setup the API parameters
    let params = SwitchDeltaModeParams::builder("delta_enabled_example".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .switch_delta_mode(params)
        .await
        .context("switch_delta_mode request failed")?;

    info!(?response.rate_limits, "switch_delta_mode rate limits");
    let data = response.data().await?;
    info!(?data, "switch_delta_mode data");

    Ok(())
}
