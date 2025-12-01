use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_coin_futures::{
    DerivativesTradingCoinFuturesRestApi,
    rest_api::{
        TakerBuySellVolumeContractTypeEnum, TakerBuySellVolumeParams, TakerBuySellVolumePeriodEnum,
    },
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

    // Create the DerivativesTradingCoinFutures REST API client
    let rest_client = DerivativesTradingCoinFuturesRestApi::production(rest_conf);

    // Setup the API parameters
    let params = TakerBuySellVolumeParams::builder(
        "pair_example".to_string(),
        TakerBuySellVolumeContractTypeEnum::Perpetual,
        TakerBuySellVolumePeriodEnum::Period5m,
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .taker_buy_sell_volume(params)
        .await
        .context("taker_buy_sell_volume request failed")?;

    info!(?response.rate_limits, "taker_buy_sell_volume rate limits");
    let data = response.data().await?;
    info!(?data, "taker_buy_sell_volume data");

    Ok(())
}
