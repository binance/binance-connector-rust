use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi, rest_api::GetDownloadIdForFuturesOrderHistoryParams,
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

    // Create the DerivativesTradingUsdsFutures REST API client
    let rest_client = DerivativesTradingUsdsFuturesRestApi::production(rest_conf);

    // Setup the API parameters
    let params =
        GetDownloadIdForFuturesOrderHistoryParams::builder(1623319461670, 1641782889000).build()?;

    // Make the API call
    let response = rest_client
        .get_download_id_for_futures_order_history(params)
        .await
        .context("get_download_id_for_futures_order_history request failed")?;

    info!(?response.rate_limits, "get_download_id_for_futures_order_history rate limits");
    let data = response.data().await?;
    info!(?data, "get_download_id_for_futures_order_history data");

    Ok(())
}
