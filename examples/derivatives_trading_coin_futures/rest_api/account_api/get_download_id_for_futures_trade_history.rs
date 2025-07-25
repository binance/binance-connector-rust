use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_coin_futures::{
    DerivativesTradingCoinFuturesRestApi, rest_api::GetDownloadIdForFuturesTradeHistoryParams,
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
    let params =
        GetDownloadIdForFuturesTradeHistoryParams::builder(1623319461670, 1641782889000).build()?;

    // Make the API call
    let response = rest_client
        .get_download_id_for_futures_trade_history(params)
        .await
        .context("get_download_id_for_futures_trade_history request failed")?;

    info!(?response.rate_limits, "get_download_id_for_futures_trade_history rate limits");
    let data = response.data().await?;
    info!(?data, "get_download_id_for_futures_trade_history data");

    Ok(())
}
