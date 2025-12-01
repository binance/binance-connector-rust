use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::sub_account::{
    SubAccountRestApi, rest_api::TransferToSubAccountOfSameMasterParams,
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
    let params = TransferToSubAccountOfSameMasterParams::builder(
        "to_email_example".to_string(),
        "asset_example".to_string(),
        dec!(1.0),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .transfer_to_sub_account_of_same_master(params)
        .await
        .context("transfer_to_sub_account_of_same_master request failed")?;

    info!(?response.rate_limits, "transfer_to_sub_account_of_same_master rate limits");
    let data = response.data().await?;
    info!(?data, "transfer_to_sub_account_of_same_master data");

    Ok(())
}
