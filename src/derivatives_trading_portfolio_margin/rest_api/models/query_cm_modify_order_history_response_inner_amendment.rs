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
pub struct QueryCmModifyOrderHistoryResponseInnerAmendment {
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<models::QueryCmModifyOrderHistoryResponseInnerAmendmentPrice>>,
    #[serde(rename = "origQty", skip_serializing_if = "Option::is_none")]
    pub orig_qty: Option<Box<models::QueryCmModifyOrderHistoryResponseInnerAmendmentOrigQty>>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl QueryCmModifyOrderHistoryResponseInnerAmendment {
    #[must_use]
    pub fn new() -> QueryCmModifyOrderHistoryResponseInnerAmendment {
        QueryCmModifyOrderHistoryResponseInnerAmendment {
            price: None,
            orig_qty: None,
            count: None,
        }
    }
}
