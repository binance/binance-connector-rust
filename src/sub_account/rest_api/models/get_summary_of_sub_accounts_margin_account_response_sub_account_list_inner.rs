/*
 * Binance Sub Account REST API
 *
 * OpenAPI Specification for the Binance Sub Account REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::sub_account::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSummaryOfSubAccountsMarginAccountResponseSubAccountListInner {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "totalAssetOfBtc", skip_serializing_if = "Option::is_none")]
    pub total_asset_of_btc: Option<String>,
    #[serde(
        rename = "totalLiabilityOfBtc",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_liability_of_btc: Option<String>,
    #[serde(rename = "totalNetAssetOfBtc", skip_serializing_if = "Option::is_none")]
    pub total_net_asset_of_btc: Option<String>,
}

impl GetSummaryOfSubAccountsMarginAccountResponseSubAccountListInner {
    #[must_use]
    pub fn new() -> GetSummaryOfSubAccountsMarginAccountResponseSubAccountListInner {
        GetSummaryOfSubAccountsMarginAccountResponseSubAccountListInner {
            email: None,
            total_asset_of_btc: None,
            total_liability_of_btc: None,
            total_net_asset_of_btc: None,
        }
    }
}
