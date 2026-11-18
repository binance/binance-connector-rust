use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::stock_options::{StockOptionsRestApi, rest_api::SubmitOptionExerciseParams};

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

    // Create the StockOptions REST API client
    let rest_client = StockOptionsRestApi::production(rest_conf);

    // Setup the API parameters
    let params =
        SubmitOptionExerciseParams::builder("NVDA260930C00100000".to_string(), "1".to_string())
            .build()?;

    // Make the API call
    let response = rest_client
        .submit_option_exercise(params)
        .await
        .context("submit_option_exercise request failed")?;

    info!(?response.rate_limits, "submit_option_exercise rate limits");
    let data = response.data().await?;
    info!(?data, "submit_option_exercise data");

    Ok(())
}
