/*
 * Binance Crypto Loan REST API
 *
 * OpenAPI Specification for the Binance Crypto Loan REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::crypto_loan::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLoanLtvAdjustmentHistoryResponseRowsInner {
    #[serde(rename = "loanCoin", skip_serializing_if = "Option::is_none")]
    pub loan_coin: Option<String>,
    #[serde(rename = "collateralCoin", skip_serializing_if = "Option::is_none")]
    pub collateral_coin: Option<String>,
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "preLTV", skip_serializing_if = "Option::is_none")]
    pub pre_ltv: Option<String>,
    #[serde(rename = "afterLTV", skip_serializing_if = "Option::is_none")]
    pub after_ltv: Option<String>,
    #[serde(rename = "adjustTime", skip_serializing_if = "Option::is_none")]
    pub adjust_time: Option<i64>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
}

impl GetLoanLtvAdjustmentHistoryResponseRowsInner {
    #[must_use]
    pub fn new() -> GetLoanLtvAdjustmentHistoryResponseRowsInner {
        GetLoanLtvAdjustmentHistoryResponseRowsInner {
            loan_coin: None,
            collateral_coin: None,
            direction: None,
            amount: None,
            pre_ltv: None,
            after_ltv: None,
            adjust_time: None,
            order_id: None,
        }
    }
}
