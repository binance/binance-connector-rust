use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_options::{
    DerivativesTradingOptionsRestApi, rest_api::SetMarketMakerProtectionConfigParams,
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
    let params = SetMarketMakerProtectionConfigParams::default();

    // Make the API call
    let response = rest_client
        .set_market_maker_protection_config(params)
        .await
        .context("set_market_maker_protection_config request failed")?;

    info!(?response.rate_limits, "set_market_maker_protection_config rate limits");
    let data = response.data().await?;
    info!(?data, "set_market_maker_protection_config data");

    Ok(())
}
