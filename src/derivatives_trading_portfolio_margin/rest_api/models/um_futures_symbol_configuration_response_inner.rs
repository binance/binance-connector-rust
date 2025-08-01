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
pub struct UmFuturesSymbolConfigurationResponseInner {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "marginType", skip_serializing_if = "Option::is_none")]
    pub margin_type: Option<String>,
    #[serde(rename = "isAutoAddMargin", skip_serializing_if = "Option::is_none")]
    pub is_auto_add_margin: Option<String>,
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: Option<i64>,
    #[serde(rename = "maxNotionalValue", skip_serializing_if = "Option::is_none")]
    pub max_notional_value: Option<String>,
}

impl UmFuturesSymbolConfigurationResponseInner {
    #[must_use]
    pub fn new() -> UmFuturesSymbolConfigurationResponseInner {
        UmFuturesSymbolConfigurationResponseInner {
            symbol: None,
            margin_type: None,
            is_auto_add_margin: None,
            leverage: None,
            max_notional_value: None,
        }
    }
}
