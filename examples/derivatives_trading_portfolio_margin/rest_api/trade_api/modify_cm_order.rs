use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_portfolio_margin::{
    DerivativesTradingPortfolioMarginRestApi,
    rest_api::{ModifyCmOrderParams, ModifyCmOrderSideEnum},
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
    let params = ModifyCmOrderParams::builder(
        "symbol_example".to_string(),
        ModifyCmOrderSideEnum::Buy,
        dec!(1.0),
        dec!(1.0),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .modify_cm_order(params)
        .await
        .context("modify_cm_order request failed")?;

    info!(?response.rate_limits, "modify_cm_order rate limits");
    let data = response.data().await?;
    info!(?data, "modify_cm_order data");

    Ok(())
}
