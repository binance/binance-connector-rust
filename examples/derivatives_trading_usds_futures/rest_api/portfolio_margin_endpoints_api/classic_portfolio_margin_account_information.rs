use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi, rest_api::ClassicPortfolioMarginAccountInformationParams,
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

    // Create the DerivativesTradingUsdsFutures REST API client
    let rest_client = DerivativesTradingUsdsFuturesRestApi::production(rest_conf);

    // Setup the API parameters
    let params =
        ClassicPortfolioMarginAccountInformationParams::builder("asset_example".to_string())
            .build()?;

    // Make the API call
    let response = rest_client
        .classic_portfolio_margin_account_information(params)
        .await
        .context("classic_portfolio_margin_account_information request failed")?;

    info!(?response.rate_limits, "classic_portfolio_margin_account_information rate limits");
    let data = response.data().await?;
    info!(?data, "classic_portfolio_margin_account_information data");

    Ok(())
}
