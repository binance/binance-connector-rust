use anyhow::{Context, Result};
use tokio::time::{Duration, sleep};
use tracing::info;

use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::derivatives_trading_coin_futures::{
    DerivativesTradingCoinFuturesWsStreams, websocket_streams::IndexKlineCandlestickStreamsParams,
};
use binance_sdk::logger;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise logging
    logger::init();

    // Build WebSocket Streams config
    let ws_streams_conf = ConfigurationWebsocketStreams::builder().build()?;

    // Create the DerivativesTradingCoinFutures WebSocket Streams client
    let ws_streams_client = DerivativesTradingCoinFuturesWsStreams::production(ws_streams_conf);

    // Connect to WebSocket
    let connection = ws_streams_client
        .connect()
        .await
        .context("Failed to connect to WebSocket Streams")?;

    // Setup the stream parameters
    let params =
        IndexKlineCandlestickStreamsParams::builder("btcusdt".to_string(), "1m".to_string())
            .build()?;

    // Subscribe to the stream
    let stream = connection
        .index_kline_candlestick_streams(params)
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
