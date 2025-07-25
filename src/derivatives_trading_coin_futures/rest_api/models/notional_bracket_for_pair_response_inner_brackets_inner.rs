/*
 * Binance Derivatives Trading COIN Futures REST API
 *
 * OpenAPI Specification for the Binance Derivatives Trading COIN Futures REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::derivatives_trading_coin_futures::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotionalBracketForPairResponseInnerBracketsInner {
    #[serde(rename = "bracket", skip_serializing_if = "Option::is_none")]
    pub bracket: Option<i64>,
    #[serde(rename = "initialLeverage", skip_serializing_if = "Option::is_none")]
    pub initial_leverage: Option<i64>,
    #[serde(rename = "qtyCap", skip_serializing_if = "Option::is_none")]
    pub qty_cap: Option<i64>,
    #[serde(rename = "qtylFloor", skip_serializing_if = "Option::is_none")]
    pub qtyl_floor: Option<i64>,
    #[serde(rename = "maintMarginRatio", skip_serializing_if = "Option::is_none")]
    pub maint_margin_ratio: Option<rust_decimal::Decimal>,
    #[serde(rename = "cum", skip_serializing_if = "Option::is_none")]
    pub cum: Option<rust_decimal::Decimal>,
}

impl NotionalBracketForPairResponseInnerBracketsInner {
    #[must_use]
    pub fn new() -> NotionalBracketForPairResponseInnerBracketsInner {
        NotionalBracketForPairResponseInnerBracketsInner {
            bracket: None,
            initial_leverage: None,
            qty_cap: None,
            qtyl_floor: None,
            maint_margin_ratio: None,
            cum: None,
        }
    }
}
