use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::spot::{
    SpotRestApi,
    rest_api::{
        OrderListOcoAboveTypeEnum, OrderListOcoBelowTypeEnum, OrderListOcoParams,
        OrderListOcoSideEnum,
    },
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
    let params = OrderListOcoParams::builder(
        "BNBUSDT".to_string(),
        OrderListOcoSideEnum::Buy,
        dec!(1.0),
        OrderListOcoAboveTypeEnum::StopLossLimit,
        OrderListOcoBelowTypeEnum::StopLoss,
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .order_list_oco(params)
        .await
        .context("order_list_oco request failed")?;

    info!(?response.rate_limits, "order_list_oco rate limits");
    let data = response.data().await?;
    info!(?data, "order_list_oco data");

    Ok(())
}
