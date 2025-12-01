use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::sub_account::{SubAccountRestApi, rest_api::QuerySubAccountAssetsParams};

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

    // Create the SubAccount REST API client
    let rest_client = SubAccountRestApi::production(rest_conf);

    // Setup the API parameters
    let params =
        QuerySubAccountAssetsParams::builder("sub-account-email@email.com".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .query_sub_account_assets(params)
        .await
        .context("query_sub_account_assets request failed")?;

    info!(?response.rate_limits, "query_sub_account_assets rate limits");
    let data = response.data().await?;
    info!(?data, "query_sub_account_assets data");

    Ok(())
}
