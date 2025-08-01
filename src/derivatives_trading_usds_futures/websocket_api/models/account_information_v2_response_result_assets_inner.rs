/*
 * Binance Derivatives Trading USDS Futures WebSocket API
 *
 * OpenAPI Specification for the Binance Derivatives Trading USDS Futures WebSocket API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::derivatives_trading_usds_futures::websocket_api::models;
use serde::{Deserialize, Deserializer, Serialize, de::Error};
use serde_json::Value;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountInformationV2ResponseResultAssetsInner {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "walletBalance", skip_serializing_if = "Option::is_none")]
    pub wallet_balance: Option<String>,
    #[serde(rename = "unrealizedProfit", skip_serializing_if = "Option::is_none")]
    pub unrealized_profit: Option<String>,
    #[serde(rename = "marginBalance", skip_serializing_if = "Option::is_none")]
    pub margin_balance: Option<String>,
    #[serde(rename = "maintMargin", skip_serializing_if = "Option::is_none")]
    pub maint_margin: Option<String>,
    #[serde(rename = "initialMargin", skip_serializing_if = "Option::is_none")]
    pub initial_margin: Option<String>,
    #[serde(
        rename = "positionInitialMargin",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_initial_margin: Option<String>,
    #[serde(
        rename = "openOrderInitialMargin",
        skip_serializing_if = "Option::is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    #[serde(rename = "crossWalletBalance", skip_serializing_if = "Option::is_none")]
    pub cross_wallet_balance: Option<String>,
    #[serde(rename = "crossUnPnl", skip_serializing_if = "Option::is_none")]
    pub cross_un_pnl: Option<String>,
    #[serde(rename = "availableBalance", skip_serializing_if = "Option::is_none")]
    pub available_balance: Option<String>,
    #[serde(rename = "maxWithdrawAmount", skip_serializing_if = "Option::is_none")]
    pub max_withdraw_amount: Option<String>,
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(rename = "marginAvailable", skip_serializing_if = "Option::is_none")]
    pub margin_available: Option<bool>,
}

impl AccountInformationV2ResponseResultAssetsInner {
    #[must_use]
    pub fn new() -> AccountInformationV2ResponseResultAssetsInner {
        AccountInformationV2ResponseResultAssetsInner {
            asset: None,
            wallet_balance: None,
            unrealized_profit: None,
            margin_balance: None,
            maint_margin: None,
            initial_margin: None,
            position_initial_margin: None,
            open_order_initial_margin: None,
            cross_wallet_balance: None,
            cross_un_pnl: None,
            available_balance: None,
            max_withdraw_amount: None,
            update_time: None,
            margin_available: None,
        }
    }
}
