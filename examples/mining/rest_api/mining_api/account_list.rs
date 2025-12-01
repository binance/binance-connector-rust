use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::mining::{MiningRestApi, rest_api::AccountListParams};

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

    // Create the Mining REST API client
    let rest_client = MiningRestApi::production(rest_conf);

    // Setup the API parameters
    let params =
        AccountListParams::builder("algo_example".to_string(), "user_name_example".to_string())
            .build()?;

    // Make the API call
    let response = rest_client
        .account_list(params)
        .await
        .context("account_list request failed")?;

    info!(?response.rate_limits, "account_list rate limits");
    let data = response.data().await?;
    info!(?data, "account_list data");

    Ok(())
}
