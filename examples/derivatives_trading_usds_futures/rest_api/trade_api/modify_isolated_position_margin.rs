use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi, rest_api::ModifyIsolatedPositionMarginParams,
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

    // Create the DerivativesTradingUsdsFutures REST API client
    let rest_client = DerivativesTradingUsdsFuturesRestApi::production(rest_conf);

    // Setup the API parameters
    let params = ModifyIsolatedPositionMarginParams::builder(
        "symbol_example".to_string(),
        dec!(1.0),
        "r#type_example".to_string(),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .modify_isolated_position_margin(params)
        .await
        .context("modify_isolated_position_margin request failed")?;

    info!(?response.rate_limits, "modify_isolated_position_margin rate limits");
    let data = response.data().await?;
    info!(?data, "modify_isolated_position_margin data");

    Ok(())
}
