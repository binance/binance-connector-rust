use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::sub_account::{
    SubAccountRestApi, rest_api::DeleteIpListForASubAccountApiKeyParams,
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

    // Create the SubAccount REST API client
    let rest_client = SubAccountRestApi::production(rest_conf);

    // Setup the API parameters
    let params = DeleteIpListForASubAccountApiKeyParams::builder(
        "sub-account-email@email.com".to_string(),
        "sub_account_api_key_example".to_string(),
        "ip_address_example".to_string(),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .delete_ip_list_for_a_sub_account_api_key(params)
        .await
        .context("delete_ip_list_for_a_sub_account_api_key request failed")?;

    info!(?response.rate_limits, "delete_ip_list_for_a_sub_account_api_key rate limits");
    let data = response.data().await?;
    info!(?data, "delete_ip_list_for_a_sub_account_api_key data");

    Ok(())
}
