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
pub struct AccountInformationResponse {
    #[serde(rename = "uniMMR", skip_serializing_if = "Option::is_none")]
    pub uni_mmr: Option<String>,
    #[serde(rename = "accountEquity", skip_serializing_if = "Option::is_none")]
    pub account_equity: Option<String>,
    #[serde(rename = "actualEquity", skip_serializing_if = "Option::is_none")]
    pub actual_equity: Option<String>,
    #[serde(
        rename = "accountInitialMargin",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_initial_margin: Option<String>,
    #[serde(rename = "accountMaintMargin", skip_serializing_if = "Option::is_none")]
    pub account_maint_margin: Option<String>,
    #[serde(rename = "accountStatus", skip_serializing_if = "Option::is_none")]
    pub account_status: Option<String>,
    #[serde(
        rename = "virtualMaxWithdrawAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_max_withdraw_amount: Option<String>,
    #[serde(
        rename = "totalAvailableBalance",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_available_balance: Option<String>,
    #[serde(
        rename = "totalMarginOpenLoss",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_margin_open_loss: Option<String>,
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

impl AccountInformationResponse {
    #[must_use]
    pub fn new() -> AccountInformationResponse {
        AccountInformationResponse {
            uni_mmr: None,
            account_equity: None,
            actual_equity: None,
            account_initial_margin: None,
            account_maint_margin: None,
            account_status: None,
            virtual_max_withdraw_amount: None,
            total_available_balance: None,
            total_margin_open_loss: None,
            update_time: None,
        }
    }
}
