use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::sub_account::{
    SubAccountRestApi, rest_api::QueryManagedSubAccountTransferLogMasterAccountInvestorParams,
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
    let params = QueryManagedSubAccountTransferLogMasterAccountInvestorParams::builder(
        "sub-account-email@email.com".to_string(),
        1623319461670,
        1641782889000,
        789,
        789,
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .query_managed_sub_account_transfer_log_master_account_investor(params)
        .await
        .context("query_managed_sub_account_transfer_log_master_account_investor request failed")?;

    info!(?response.rate_limits, "query_managed_sub_account_transfer_log_master_account_investor rate limits");
    let data = response.data().await?;
    info!(
        ?data,
        "query_managed_sub_account_transfer_log_master_account_investor data"
    );

    Ok(())
}
