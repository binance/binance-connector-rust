use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationWebsocketApi;
use binance_sdk::config::PrivateKey;
use binance_sdk::logger;
use binance_sdk::spot::{SpotWsApi, websocket_api::SessionLogonParams};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise logging
    logger::init();

    // Load credentials from env
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the environment");

    // Build WebSocket API config
    let ws_api_conf = ConfigurationWebsocketApi::builder()
        .api_key(api_key)
        .private_key(PrivateKey::File("your-private-key-file-path".to_string()))
        .build()?;

    // Create the Spot WebSocket API client
    let ws_api_client = SpotWsApi::production(ws_api_conf);

    // Connect to WebSocket
    let connection = ws_api_client
        .connect()
        .await
        .context("Failed to connect to WebSocket API")?;

    // Setup the WS API parameters
    let params = SessionLogonParams::default();

    // Make the WS API call
    let response = connection
        .session_logon(params)
        .await
        .context("session_logon request failed")?;

    for (idx, resp) in response.into_iter().enumerate() {
        info!(response_index = idx, ?resp.rate_limits, "session_logon rate limits");
        let data = resp.data()?;
        info!(response_index = idx, ?data, "session_logon data");
    }

    // Cleanly disconnect
    connection
        .disconnect()
        .await
        .context("Failed to disconnect WebSocket client")?;

    Ok(())
}
