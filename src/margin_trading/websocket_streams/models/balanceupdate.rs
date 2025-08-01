/*
 * Binance Margin Trading WebSocket Market Streams
 *
 * OpenAPI Specification for the Binance Margin Trading WebSocket Market Streams
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::margin_trading::websocket_streams::models;
use serde::{Deserialize, Deserializer, Serialize, de::Error};
use serde_json::Value;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Balanceupdate {
    #[serde(rename = "E", skip_serializing_if = "Option::is_none")]
    pub e_uppercase: Option<i64>,
    #[serde(rename = "a", skip_serializing_if = "Option::is_none")]
    pub a: Option<String>,
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,
    #[serde(rename = "T", skip_serializing_if = "Option::is_none")]
    pub t_uppercase: Option<i64>,
}

impl Balanceupdate {
    #[must_use]
    pub fn new() -> Balanceupdate {
        Balanceupdate {
            e_uppercase: None,
            a: None,
            d: None,
            t_uppercase: None,
        }
    }
}
