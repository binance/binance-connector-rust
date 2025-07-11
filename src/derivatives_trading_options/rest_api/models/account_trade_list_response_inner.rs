/*
 * Binance Derivatives Trading Options REST API
 *
 * OpenAPI Specification for the Binance Derivatives Trading Options REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::derivatives_trading_options::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountTradeListResponseInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<i64>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename = "realizedProfit", skip_serializing_if = "Option::is_none")]
    pub realized_profit: Option<String>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "volatility", skip_serializing_if = "Option::is_none")]
    pub volatility: Option<String>,
    #[serde(rename = "liquidity", skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    #[serde(rename = "quoteAsset", skip_serializing_if = "Option::is_none")]
    pub quote_asset: Option<String>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(rename = "priceScale", skip_serializing_if = "Option::is_none")]
    pub price_scale: Option<i64>,
    #[serde(rename = "quantityScale", skip_serializing_if = "Option::is_none")]
    pub quantity_scale: Option<i64>,
    #[serde(rename = "optionSide", skip_serializing_if = "Option::is_none")]
    pub option_side: Option<String>,
}

impl AccountTradeListResponseInner {
    #[must_use]
    pub fn new() -> AccountTradeListResponseInner {
        AccountTradeListResponseInner {
            id: None,
            trade_id: None,
            order_id: None,
            symbol: None,
            price: None,
            quantity: None,
            fee: None,
            realized_profit: None,
            side: None,
            r#type: None,
            volatility: None,
            liquidity: None,
            quote_asset: None,
            time: None,
            price_scale: None,
            quantity_scale: None,
            option_side: None,
        }
    }
}
