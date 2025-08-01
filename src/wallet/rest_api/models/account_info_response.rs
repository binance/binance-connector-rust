/*
 * Binance Wallet REST API
 *
 * OpenAPI Specification for the Binance Wallet REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::wallet::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountInfoResponse {
    #[serde(rename = "vipLevel", skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<i64>,
    #[serde(rename = "isMarginEnabled", skip_serializing_if = "Option::is_none")]
    pub is_margin_enabled: Option<bool>,
    #[serde(rename = "isFutureEnabled", skip_serializing_if = "Option::is_none")]
    pub is_future_enabled: Option<bool>,
    #[serde(rename = "isOptionsEnabled", skip_serializing_if = "Option::is_none")]
    pub is_options_enabled: Option<bool>,
    #[serde(
        rename = "isPortfolioMarginRetailEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_portfolio_margin_retail_enabled: Option<bool>,
}

impl AccountInfoResponse {
    #[must_use]
    pub fn new() -> AccountInfoResponse {
        AccountInfoResponse {
            vip_level: None,
            is_margin_enabled: None,
            is_future_enabled: None,
            is_options_enabled: None,
            is_portfolio_margin_retail_enabled: None,
        }
    }
}
