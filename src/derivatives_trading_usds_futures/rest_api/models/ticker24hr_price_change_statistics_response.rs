/*
 * Binance Derivatives Trading USDS Futures REST API
 *
 * OpenAPI Specification for the Binance Derivatives Trading USDS Futures REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::derivatives_trading_usds_futures::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Ticker24hrPriceChangeStatisticsResponse {
    Ticker24hrPriceChangeStatisticsResponse1(Box<models::Ticker24hrPriceChangeStatisticsResponse1>),
    Ticker24hrPriceChangeStatisticsResponse2(
        Vec<models::Ticker24hrPriceChangeStatisticsResponse2Inner>,
    ),
    Other(serde_json::Value),
}

impl Default for Ticker24hrPriceChangeStatisticsResponse {
    fn default() -> Self {
        Self::Ticker24hrPriceChangeStatisticsResponse1(Default::default())
    }
}
