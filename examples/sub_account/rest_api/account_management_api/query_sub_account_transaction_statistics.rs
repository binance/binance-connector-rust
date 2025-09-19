use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::sub_account::{
    SubAccountRestApi, rest_api::QuerySubAccountTransactionStatisticsParams,
};

#[tokio::main]
async fn main() -> Result<()> {
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
    let params = QuerySubAccountTransactionStatisticsParams::default();

    // Make the API call
    let response = rest_client
        .query_sub_account_transaction_statistics(params)
        .await
        .context("query_sub_account_transaction_statistics request failed")?;

    info!(?response.rate_limits, "query_sub_account_transaction_statistics rate limits");
    let data = response.data().await?;
    info!(?data, "query_sub_account_transaction_statistics data");

    Ok(())
}
