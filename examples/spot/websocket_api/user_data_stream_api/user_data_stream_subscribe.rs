use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationWebsocketApi;
use binance_sdk::logger;
use binance_sdk::spot::{SpotWsApi, websocket_api::UserDataStreamSubscribeParams};

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

    // Create the Spot WebSocket API client
    let ws_api_client = SpotWsApi::production(ws_api_conf);

    // Connect to WebSocket
    let connection = ws_api_client
        .connect()
        .await
        .context("Failed to connect to WebSocket API")?;

    // Setup the WS API parameters
    let params = UserDataStreamSubscribeParams::default();

    // Make the WS API call
    let (response, stream) = connection
        .user_data_stream_subscribe(params)
        .await
        .context("user_data_stream_subscribe request failed")?;

    info!(?response.rate_limits, "user_data_stream_subscribe rate limits");
    let data = response.data()?;
    info!(?data, "user_data_stream_subscribe data");

    stream.on_message(|data| {
        info!(?data, "user_data_stream_subscribe stream data");
    });

    // Cleanly disconnect
    connection
        .disconnect()
        .await
        .context("Failed to disconnect WebSocket client")?;

    Ok(())
}
