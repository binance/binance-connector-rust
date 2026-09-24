use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::w3w_prediction::{W3WPredictionRestApi, rest_api::GetPortfolioParams};

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

    // Create the W3WPrediction REST API client
    let rest_client = W3WPredictionRestApi::production(rest_conf);

    // Setup the API parameters
    let params =
        GetPortfolioParams::builder("0x12e32db8817e292508c34111cbc4b23340df542c".to_string())
            .build()?;

    // Make the API call
    let response = rest_client
        .get_portfolio(params)
        .await
        .context("get_portfolio request failed")?;

    info!(?response.rate_limits, "get_portfolio rate limits");
    let data = response.data().await?;
    info!(?data, "get_portfolio data");

    Ok(())
}
