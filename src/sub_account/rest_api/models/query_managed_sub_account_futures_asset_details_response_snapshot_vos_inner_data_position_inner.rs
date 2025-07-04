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
pub struct QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInnerDataPositionInner {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "entryPrice", skip_serializing_if = "Option::is_none")]
    pub entry_price: Option<i64>,
    #[serde(rename = "markPrice", skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<i64>,
    #[serde(rename = "positionAmt", skip_serializing_if = "Option::is_none")]
    pub position_amt: Option<rust_decimal::Decimal>,
}

impl QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInnerDataPositionInner {
    #[must_use]
    pub fn new()
    -> QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInnerDataPositionInner {
        QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInnerDataPositionInner {
            symbol: None,
            entry_price: None,
            mark_price: None,
            position_amt: None,
        }
    }
}
