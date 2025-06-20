/*
 * Binance Staking REST API
 *
 * OpenAPI Specification for the Binance Staking REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::staking::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedeemEthResponse {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "ethAmount", skip_serializing_if = "Option::is_none")]
    pub eth_amount: Option<String>,
    #[serde(rename = "conversionRatio", skip_serializing_if = "Option::is_none")]
    pub conversion_ratio: Option<String>,
    #[serde(rename = "arrivalTime", skip_serializing_if = "Option::is_none")]
    pub arrival_time: Option<i64>,
}

impl RedeemEthResponse {
    #[must_use]
    pub fn new() -> RedeemEthResponse {
        RedeemEthResponse {
            success: None,
            eth_amount: None,
            conversion_ratio: None,
            arrival_time: None,
        }
    }
}
