use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationWebsocketApi;
use binance_sdk::derivatives_trading_coin_futures::{
    DerivativesTradingCoinFuturesWsApi, websocket_api::StartUserDataStreamParams,
};
use binance_sdk::logger;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise logging
    logger::init();

    // Load credentials from env
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the environment");
    let api_secret = env::var("API_SECRET").expect("API_SECRET must be set in the environment");

    // Build WebSocket API config
    let ws_api_conf = ConfigurationWebsocketApi::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()?;

    // Create the DerivativesTradingCoinFutures WebSocket API client
    let ws_api_client = DerivativesTradingCoinFuturesWsApi::production(ws_api_conf);

    // Connect to WebSocket
    let connection = ws_api_client
        .connect()
        .await
        .context("Failed to connect to WebSocket API")?;

    // Setup the WS API parameters
    let params = StartUserDataStreamParams::default();

    // Make the WS API call
    let response = connection
        .start_user_data_stream(params)
        .await
        .context("start_user_data_stream request failed")?;

    info!(?response.rate_limits, "start_user_data_stream rate limits");
    let data = response.data()?;
    info!(?data, "start_user_data_stream data");

    // Cleanly disconnect
    connection
        .disconnect()
        .await
        .context("Failed to disconnect WebSocket client")?;

    Ok(())
}
