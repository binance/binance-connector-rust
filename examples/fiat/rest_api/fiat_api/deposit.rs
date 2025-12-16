use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::fiat::{FiatRestApi, rest_api::DepositParams};
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

    // Create the Fiat REST API client
    let rest_client = FiatRestApi::production(rest_conf);

    // Setup the API parameters
    let params = DepositParams::builder(
        "currency_example".to_string(),
        "api_payment_method_example".to_string(),
        789,
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .deposit(params)
        .await
        .context("deposit request failed")?;

    info!(?response.rate_limits, "deposit rate limits");
    let data = response.data().await?;
    info!(?data, "deposit data");

    Ok(())
}
