/*
 * Binance Margin Trading REST API
 *
 * OpenAPI Specification for the Binance Margin Trading REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::margin_trading::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetListScheduleResponseInner {
    #[serde(rename = "listTime", skip_serializing_if = "Option::is_none")]
    pub list_time: Option<i64>,
    #[serde(rename = "crossMarginAssets", skip_serializing_if = "Option::is_none")]
    pub cross_margin_assets: Option<Vec<String>>,
    #[serde(
        rename = "isolatedMarginSymbols",
        skip_serializing_if = "Option::is_none"
    )]
    pub isolated_margin_symbols: Option<Vec<String>>,
}

impl GetListScheduleResponseInner {
    #[must_use]
    pub fn new() -> GetListScheduleResponseInner {
        GetListScheduleResponseInner {
            list_time: None,
            cross_margin_assets: None,
            isolated_margin_symbols: None,
        }
    }
}
