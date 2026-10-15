use anyhow::{Context, Result};
use std::env;
use tokio::time::{Duration, sleep};
use tracing::info;

use binance_sdk::cms::CmsWsStreams;
use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::logger;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise logging
    logger::init();

    // Build WebSocket Streams config
    let api_key = env::var("API_KEY").context("API_KEY must be set")?;
    let api_secret = env::var("API_SECRET").context("API_SECRET must be set")?;
    let ws_streams_conf = ConfigurationWebsocketStreams::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()?;

    // Create the Cms WebSocket Streams client
    let ws_streams_client = CmsWsStreams::production(ws_streams_conf);

    // Connect to WebSocket
    let connection = ws_streams_client
        .connect()
        .await
        .context("Failed to connect to WebSocket Streams")?;

    // Subscribe to the stream
    let stream = connection
        .com_announcement_en()
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
