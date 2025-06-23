use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationWebsocketApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesWsApi,
    websocket_api::{ModifyOrderParams, ModifyOrderSideEnum},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load credentials from env
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the environment");
    let api_secret = env::var("API_SECRET").expect("API_SECRET must be set in the environment");

    // Build WebSocket API config
    let ws_api_conf = ConfigurationWebsocketApi::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()?;

    // Create the DerivativesTradingUsdsFutures WebSocket API client
    let ws_api_client = DerivativesTradingUsdsFuturesWsApi::production(ws_api_conf);

    // Connect to WebSocket
    let connection = ws_api_client
        .connect()
        .await
        .context("Failed to connect to WebSocket API")?;

    // Setup the WS API parameters
    let params = ModifyOrderParams::builder(
        "symbol_example".to_string(),
        ModifyOrderSideEnum::Buy,
        1.0,
        1.0,
    )
    .build()?;

    // Make the WS API call
    let response = connection
        .modify_order(params)
        .await
        .context("modify_order request failed")?;

    info!(?response.rate_limits, "modify_order rate limits");
    let data = response.data()?;
    info!(?data, "modify_order data");

    // Cleanly disconnect
    connection
        .disconnect()
        .await
        .context("Failed to disconnect WebSocket client")?;

    Ok(())
}
