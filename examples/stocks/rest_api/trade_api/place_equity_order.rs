use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::stocks::{
    StocksRestApi,
    rest_api::{PlaceEquityOrderOrderTypeEnum, PlaceEquityOrderParams, PlaceEquityOrderSideEnum},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise logging
    logger::init();

    // Load credentials from env
    let api_key = env::var("API_KEY").context("API_KEY must be set")?;
    let api_secret = env::var("API_SECRET").context("API_SECRET must be set")?;

    // Build REST config
    let rest_conf = ConfigurationRestApi::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()?;

    // Create the Stocks REST API client
    let rest_client = StocksRestApi::production(rest_conf);

    // Setup the API parameters
    let params = PlaceEquityOrderParams::builder(
        "AAPL".to_string(),
        PlaceEquityOrderSideEnum::Buy,
        PlaceEquityOrderOrderTypeEnum::Market,
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .place_equity_order(params)
        .await
        .context("place_equity_order request failed")?;

    info!(?response.rate_limits, "place_equity_order rate limits");
    let data = response.data().await?;
    info!(?data, "place_equity_order data");

    Ok(())
}
