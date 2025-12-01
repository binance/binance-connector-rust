use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationWebsocketApi;
use binance_sdk::logger;
use binance_sdk::spot::{
    SpotWsApi,
    websocket_api::{
        OrderListPlaceOcoAboveTypeEnum, OrderListPlaceOcoBelowTypeEnum, OrderListPlaceOcoParams,
        OrderListPlaceOcoSideEnum,
    },
};

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
    let params = OrderListPlaceOcoParams::builder(
        "BNBUSDT".to_string(),
        OrderListPlaceOcoSideEnum::Buy,
        dec!(1.0),
        OrderListPlaceOcoAboveTypeEnum::StopLossLimit,
        OrderListPlaceOcoBelowTypeEnum::StopLoss,
    )
    .build()?;

    // Make the WS API call
    let response = connection
        .order_list_place_oco(params)
        .await
        .context("order_list_place_oco request failed")?;

    info!(?response.rate_limits, "order_list_place_oco rate limits");
    let data = response.data()?;
    info!(?data, "order_list_place_oco data");

    // Cleanly disconnect
    connection
        .disconnect()
        .await
        .context("Failed to disconnect WebSocket client")?;

    Ok(())
}
