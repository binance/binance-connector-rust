use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::spot::{
    SpotRestApi,
    rest_api::{UiKlinesIntervalEnum, UiKlinesParams},
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

    // Create the Spot REST API client
    let rest_client = SpotRestApi::production(rest_conf);

    // Setup the API parameters
    let params =
        UiKlinesParams::builder("BNBUSDT".to_string(), UiKlinesIntervalEnum::Interval1s).build()?;

    // Make the API call
    let response = rest_client
        .ui_klines(params)
        .await
        .context("ui_klines request failed")?;

    info!(?response.rate_limits, "ui_klines rate limits");
    let data = response.data().await?;
    info!(?data, "ui_klines data");

    Ok(())
}
