/*
 * Binance Derivatives Trading Options WebSocket Market Streams
 *
 * OpenAPI Specification for the Binance Derivatives Trading Options WebSocket Market Streams
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::derivatives_trading_options::websocket_streams::models;
use serde::{Deserialize, Deserializer, Serialize, de::Error};
use serde_json::Value;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountUpdate {
    #[serde(rename = "E", skip_serializing_if = "Option::is_none")]
    pub e_uppercase: Option<i64>,
    #[serde(rename = "B", skip_serializing_if = "Option::is_none")]
    pub b_uppercase: Option<Vec<models::AccountUpdateBInner>>,
    #[serde(rename = "G", skip_serializing_if = "Option::is_none")]
    pub g_uppercase: Option<Vec<models::AccountUpdateGInner>>,
    #[serde(rename = "P", skip_serializing_if = "Option::is_none")]
    pub p_uppercase: Option<Vec<models::AccountUpdatePInner>>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
}

impl AccountUpdate {
    #[must_use]
    pub fn new() -> AccountUpdate {
        AccountUpdate {
            e_uppercase: None,
            b_uppercase: None,
            g_uppercase: None,
            p_uppercase: None,
            uid: None,
        }
    }
}
