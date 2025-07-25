/*
 * Binance NFT REST API
 *
 * OpenAPI Specification for the Binance NFT REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::nft::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetNftDepositHistoryResponseListInner {
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "txID", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    #[serde(rename = "contractAdrress", skip_serializing_if = "Option::is_none")]
    pub contract_adrress: Option<String>,
    #[serde(rename = "tokenId", skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

impl GetNftDepositHistoryResponseListInner {
    #[must_use]
    pub fn new() -> GetNftDepositHistoryResponseListInner {
        GetNftDepositHistoryResponseListInner {
            network: None,
            tx_id: None,
            contract_adrress: None,
            token_id: None,
            timestamp: None,
        }
    }
}
