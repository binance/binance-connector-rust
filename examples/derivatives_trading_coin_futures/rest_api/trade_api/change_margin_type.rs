use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_coin_futures::{
    DerivativesTradingCoinFuturesRestApi,
    rest_api::{ChangeMarginTypeMarginTypeEnum, ChangeMarginTypeParams},
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
    let params = ChangeMarginTypeParams::builder(
        "symbol_example".to_string(),
        ChangeMarginTypeMarginTypeEnum::Isolated,
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .change_margin_type(params)
        .await
        .context("change_margin_type request failed")?;

    info!(?response.rate_limits, "change_margin_type rate limits");
    let data = response.data().await?;
    info!(?data, "change_margin_type data");

    Ok(())
}
