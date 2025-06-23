use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_coin_futures::{
    DerivativesTradingCoinFuturesRestApi,
    rest_api::{LongShortRatioParams, LongShortRatioPeriodEnum},
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

    // Create the DerivativesTradingCoinFutures REST API client
    let rest_client = DerivativesTradingCoinFuturesRestApi::production(rest_conf);

    // Setup the API parameters
    let params = LongShortRatioParams::builder(
        "pair_example".to_string(),
        LongShortRatioPeriodEnum::Period5m,
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .long_short_ratio(params)
        .await
        .context("long_short_ratio request failed")?;

    info!(?response.rate_limits, "long_short_ratio rate limits");
    let data = response.data().await?;
    info!(?data, "long_short_ratio data");

    Ok(())
}
