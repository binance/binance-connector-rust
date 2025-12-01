use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::gift_card::{GiftCardRestApi, rest_api::RedeemABinanceGiftCardParams};
use binance_sdk::logger;

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

    // Create the GiftCard REST API client
    let rest_client = GiftCardRestApi::production(rest_conf);

    // Setup the API parameters
    let params = RedeemABinanceGiftCardParams::builder("code_example".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .redeem_a_binance_gift_card(params)
        .await
        .context("redeem_a_binance_gift_card request failed")?;

    info!(?response.rate_limits, "redeem_a_binance_gift_card rate limits");
    let data = response.data().await?;
    info!(?data, "redeem_a_binance_gift_card data");

    Ok(())
}
