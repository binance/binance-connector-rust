use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::dual_investment::{
    DualInvestmentRestApi, rest_api::SubscribeDualInvestmentProductsParams,
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

    // Create the DualInvestment REST API client
    let rest_client = DualInvestmentRestApi::production(rest_conf);

    // Setup the API parameters
    let params = SubscribeDualInvestmentProductsParams::builder(
        "id_example".to_string(),
        "1".to_string(),
        dec!(1.0),
        "NONE".to_string(),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .subscribe_dual_investment_products(params)
        .await
        .context("subscribe_dual_investment_products request failed")?;

    info!(?response.rate_limits, "subscribe_dual_investment_products rate limits");
    let data = response.data().await?;
    info!(?data, "subscribe_dual_investment_products data");

    Ok(())
}
