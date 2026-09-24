use anyhow::{Context, Result};
use tokio::time::{Duration, sleep};
use tracing::info;

use binance_sdk::alpha::{
    AlphaWsStreams,
    websocket_streams::{ContractKlineStreamIntervalEnum, ContractKlineStreamParams},
};
use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::logger;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise logging
    logger::init();

    // Build WebSocket Streams config
    let ws_streams_conf = ConfigurationWebsocketStreams::builder().build()?;

    // Create the Alpha WebSocket Streams client
    let ws_streams_client = AlphaWsStreams::production(ws_streams_conf);

    // Connect to WebSocket
    let connection = ws_streams_client
        .connect()
        .await
        .context("Failed to connect to WebSocket Streams")?;

    // Setup the stream parameters
    let params = ContractKlineStreamParams::builder(
        "G7vQWurMkMMm2dU3iZpXYFTHT9Biio4F4gZCrwFpKNwG".to_string(),
        "CT_501".to_string(),
        ContractKlineStreamIntervalEnum::Interval1s,
    )
    .build()?;

    // Subscribe to the stream
    let stream = connection
        .contract_kline_stream(params)
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
