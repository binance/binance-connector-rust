// Class name: websocket_market_streams_api
use anyhow::{Context, Result};
use tokio::time::{Duration, sleep};
use tracing::info;

use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesWsStreams, websocket_streams::AllBookTickersStreamParams,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Build WebSocket Streams config
    let ws_streams_conf = ConfigurationWebsocketStreams::builder().build()?;

    // Create the DerivativesTradingUsdsFutures WebSocket Streams client
    let ws_streams_client = DerivativesTradingUsdsFuturesWsStreams::production(ws_streams_conf);

    // Connect to WebSocket
    let connection = ws_streams_client
        .connect()
        .await
        .context("Failed to connect to WebSocket Streams")?;

    // Setup the stream parameters
    let params = AllBookTickersStreamParams::default();

    // Subscribe to the stream
    let stream = connection
        .all_book_tickers_stream(params)
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
