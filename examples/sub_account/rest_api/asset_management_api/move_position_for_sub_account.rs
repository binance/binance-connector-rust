use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::logger;
use binance_sdk::sub_account::{SubAccountRestApi, rest_api::MovePositionForSubAccountParams};

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
    let params = MovePositionForSubAccountParams::builder(
        "from_user_email_example".to_string(),
        "to_user_email_example".to_string(),
        "product_type_example".to_string(),
        vec![],
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .move_position_for_sub_account(params)
        .await
        .context("move_position_for_sub_account request failed")?;

    info!(?response.rate_limits, "move_position_for_sub_account rate limits");
    let data = response.data().await?;
    info!(?data, "move_position_for_sub_account data");

    Ok(())
}
