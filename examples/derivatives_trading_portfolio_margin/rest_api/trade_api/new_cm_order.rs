use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_portfolio_margin::{
    DerivativesTradingPortfolioMarginRestApi,
    rest_api::{NewCmOrderParams, NewCmOrderSideEnum, NewCmOrderTypeEnum},
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

    // Create the DerivativesTradingPortfolioMargin REST API client
    let rest_client = DerivativesTradingPortfolioMarginRestApi::production(rest_conf);

    // Setup the API parameters
    let params = NewCmOrderParams::builder(
        "symbol_example".to_string(),
        NewCmOrderSideEnum::Buy,
        NewCmOrderTypeEnum::Limit,
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .new_cm_order(params)
        .await
        .context("new_cm_order request failed")?;

    info!(?response.rate_limits, "new_cm_order rate limits");
    let data = response.data().await?;
    info!(?data, "new_cm_order data");

    Ok(())
}
