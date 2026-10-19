use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_portfolio_margin::{
    DerivativesTradingPortfolioMarginRestApi, rest_api::UmFuturesSymbolConfigurationParams,
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

    // Create the DerivativesTradingPortfolioMargin REST API client
    let rest_client = DerivativesTradingPortfolioMarginRestApi::production(rest_conf);

    // Setup the API parameters
    let params = UmFuturesSymbolConfigurationParams::default();

    // Make the API call
    let response = rest_client
        .um_futures_symbol_configuration(params)
        .await
        .context("um_futures_symbol_configuration request failed")?;

    info!(?response.rate_limits, "um_futures_symbol_configuration rate limits");
    let data = response.data().await?;
    info!(?data, "um_futures_symbol_configuration data");

    Ok(())
}
