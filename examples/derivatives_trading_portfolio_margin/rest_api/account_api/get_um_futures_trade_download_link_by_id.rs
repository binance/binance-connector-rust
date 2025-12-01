use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_portfolio_margin::{
    DerivativesTradingPortfolioMarginRestApi, rest_api::GetUmFuturesTradeDownloadLinkByIdParams,
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
    let params = GetUmFuturesTradeDownloadLinkByIdParams::builder("1".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .get_um_futures_trade_download_link_by_id(params)
        .await
        .context("get_um_futures_trade_download_link_by_id request failed")?;

    info!(?response.rate_limits, "get_um_futures_trade_download_link_by_id rate limits");
    let data = response.data().await?;
    info!(?data, "get_um_futures_trade_download_link_by_id data");

    Ok(())
}
