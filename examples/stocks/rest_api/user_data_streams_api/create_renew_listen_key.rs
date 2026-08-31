use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::stocks::{StocksRestApi, rest_api::CreateRenewListenKeyParams};

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
    let params = CreateRenewListenKeyParams::default();

    // Make the API call
    let response = rest_client
        .create_renew_listen_key(params)
        .await
        .context("create_renew_listen_key request failed")?;

    info!(?response.rate_limits, "create_renew_listen_key rate limits");
    let data = response.data().await?;
    info!(?data, "create_renew_listen_key data");

    Ok(())
}
