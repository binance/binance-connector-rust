use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::gift_card::{
    GiftCardRestApi, rest_api::VerifyBinanceGiftCardByGiftCardNumberParams,
};
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
    let params =
        VerifyBinanceGiftCardByGiftCardNumberParams::builder("reference_no_example".to_string())
            .build()?;

    // Make the API call
    let response = rest_client
        .verify_binance_gift_card_by_gift_card_number(params)
        .await
        .context("verify_binance_gift_card_by_gift_card_number request failed")?;

    info!(?response.rate_limits, "verify_binance_gift_card_by_gift_card_number rate limits");
    let data = response.data().await?;
    info!(?data, "verify_binance_gift_card_by_gift_card_number data");

    Ok(())
}
