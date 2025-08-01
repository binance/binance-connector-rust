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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SymbolOrderBookTickerResponse {
    SymbolOrderBookTickerResponse1(Box<models::SymbolOrderBookTickerResponse1>),
    SymbolOrderBookTickerResponse2(Box<models::SymbolOrderBookTickerResponse2>),
    Other(serde_json::Value),
}

impl Default for SymbolOrderBookTickerResponse {
    fn default() -> Self {
        Self::SymbolOrderBookTickerResponse1(Default::default())
    }
}
