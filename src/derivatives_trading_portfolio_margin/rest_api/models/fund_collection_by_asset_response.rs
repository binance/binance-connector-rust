/*
 * Binance Derivatives Trading Portfolio Margin REST API
 *
 * OpenAPI Specification for the Binance Derivatives Trading Portfolio Margin REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::derivatives_trading_portfolio_margin::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FundCollectionByAssetResponse {
    #[serde(rename = "msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

impl FundCollectionByAssetResponse {
    #[must_use]
    pub fn new() -> FundCollectionByAssetResponse {
        FundCollectionByAssetResponse { msg: None }
    }
}
