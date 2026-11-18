use anyhow::{Context, Result};
use tokio::time::{Duration, sleep};
use tracing::info;

use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::logger;
use binance_sdk::stock_options::{
    StockOptionsWsStreams, websocket_streams::OrderReportV2StreamParams,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise logging
    logger::init();

    // Build WebSocket Streams config
    let ws_streams_conf = ConfigurationWebsocketStreams::builder().build()?;

    // Create the StockOptions WebSocket Streams client
    let ws_streams_client = StockOptionsWsStreams::production(ws_streams_conf);

    // Connect to WebSocket
    let connection = ws_streams_client
        .connect()
        .await
        .context("Failed to connect to WebSocket Streams")?;

    // Setup the stream parameters
    let params =
        OrderReportV2StreamParams::builder("3f9a1c2e-7b4d-4e8a-9c61-0d2f5b7e8a13".to_string())
            .build()?;

    // Subscribe to the stream
    let stream = connection
        .order_report_v2_stream(params)
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
