use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_portfolio_margin::{
    DerivativesTradingPortfolioMarginRestApi,
    rest_api::{NewUmAlgoOrderParams, NewUmAlgoOrderSideEnum, NewUmAlgoOrderTypeEnum},
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
    let params = NewUmAlgoOrderParams::builder(
        "algo_type_example".to_string(),
        "symbol_example".to_string(),
        NewUmAlgoOrderSideEnum::Buy,
        NewUmAlgoOrderTypeEnum::Limit,
        dec!(1.0),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .new_um_algo_order(params)
        .await
        .context("new_um_algo_order request failed")?;

    info!(?response.rate_limits, "new_um_algo_order rate limits");
    let data = response.data().await?;
    info!(?data, "new_um_algo_order data");

    Ok(())
}
