use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::dual_investment::{
    DualInvestmentRestApi, rest_api::GetDualInvestmentProductListParams,
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
    let params = GetDualInvestmentProductListParams::builder(
        "option_type_example".to_string(),
        "exercised_coin_example".to_string(),
        "invest_coin_example".to_string(),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .get_dual_investment_product_list(params)
        .await
        .context("get_dual_investment_product_list request failed")?;

    info!(?response.rate_limits, "get_dual_investment_product_list rate limits");
    let data = response.data().await?;
    info!(?data, "get_dual_investment_product_list data");

    Ok(())
}
