use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::copy_trading::{CopyTradingRestApi, rest_api::GetFuturesLeadTraderStatusParams};

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

    // Create the CopyTrading REST API client
    let rest_client = CopyTradingRestApi::production(rest_conf);

    // Setup the API parameters
    let params = GetFuturesLeadTraderStatusParams::default();

    // Make the API call
    let response = rest_client
        .get_futures_lead_trader_status(params)
        .await
        .context("get_futures_lead_trader_status request failed")?;

    info!(?response.rate_limits, "get_futures_lead_trader_status rate limits");
    let data = response.data().await?;
    info!(?data, "get_futures_lead_trader_status data");

    Ok(())
}
