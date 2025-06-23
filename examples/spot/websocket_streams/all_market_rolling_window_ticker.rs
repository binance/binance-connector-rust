// Class name: web_socket_streams_api
use anyhow::{Context, Result};
use tokio::time::{Duration, sleep};
use tracing::info;

use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::spot::{
    SpotWsStreams,
    websocket_streams::{
        AllMarketRollingWindowTickerParams, AllMarketRollingWindowTickerWindowSizeEnum,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // Build WebSocket Streams config
    let ws_streams_conf = ConfigurationWebsocketStreams::builder().build()?;

    // Create the Spot WebSocket Streams client
    let ws_streams_client = SpotWsStreams::production(ws_streams_conf);

    // Connect to WebSocket
    let connection = ws_streams_client
        .connect()
        .await
        .context("Failed to connect to WebSocket Streams")?;

    // Setup the stream parameters
    let params = AllMarketRollingWindowTickerParams::builder(
        AllMarketRollingWindowTickerWindowSizeEnum::WindowSize1h,
    )
    .build()?;

    // Subscribe to the stream
    let stream = connection
        .all_market_rolling_window_ticker(params)
        .await
        .context("Failed to subscribe to the stream")?;

    // Register callback for incoming messages
    stream.on_message(|data| {
        info!("{:?}", data);
    });

    // Disconnect after 20 seconds
    sleep(Duration::from_secs(20)).await;
    connection
        .disconnect()
        .await
        .context("Failed to disconnect WebSocket client")?;

    Ok(())
}
