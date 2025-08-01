/*
 * Binance Derivatives Trading USDS Futures WebSocket Market Streams
 *
 * OpenAPI Specification for the Binance Derivatives Trading USDS Futures WebSocket Market Streams
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::derivatives_trading_usds_futures::websocket_streams::models;
use serde::{Deserialize, Deserializer, Serialize, de::Error};
use serde_json::Value;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderTradeUpdateO {
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(rename = "S", skip_serializing_if = "Option::is_none")]
    pub s_uppercase: Option<String>,
    #[serde(rename = "o", skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,
    #[serde(rename = "f", skip_serializing_if = "Option::is_none")]
    pub f: Option<String>,
    #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(rename = "p", skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[serde(rename = "ap", skip_serializing_if = "Option::is_none")]
    pub ap: Option<String>,
    #[serde(rename = "sp", skip_serializing_if = "Option::is_none")]
    pub sp: Option<String>,
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(rename = "X", skip_serializing_if = "Option::is_none")]
    pub x_uppercase: Option<String>,
    #[serde(rename = "i", skip_serializing_if = "Option::is_none")]
    pub i: Option<i64>,
    #[serde(rename = "l", skip_serializing_if = "Option::is_none")]
    pub l: Option<String>,
    #[serde(rename = "z", skip_serializing_if = "Option::is_none")]
    pub z: Option<String>,
    #[serde(rename = "L", skip_serializing_if = "Option::is_none")]
    pub l_uppercase: Option<String>,
    #[serde(rename = "N", skip_serializing_if = "Option::is_none")]
    pub n_uppercase: Option<String>,
    #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[serde(rename = "T", skip_serializing_if = "Option::is_none")]
    pub t_uppercase: Option<i64>,
    #[serde(rename = "t", skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    #[serde(rename = "b", skip_serializing_if = "Option::is_none")]
    pub b: Option<String>,
    #[serde(rename = "a", skip_serializing_if = "Option::is_none")]
    pub a: Option<String>,
    #[serde(rename = "m", skip_serializing_if = "Option::is_none")]
    pub m: Option<bool>,
    #[serde(rename = "R", skip_serializing_if = "Option::is_none")]
    pub r_uppercase: Option<bool>,
    #[serde(rename = "wt", skip_serializing_if = "Option::is_none")]
    pub wt: Option<String>,
    #[serde(rename = "ot", skip_serializing_if = "Option::is_none")]
    pub ot: Option<String>,
    #[serde(rename = "ps", skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    #[serde(rename = "cp", skip_serializing_if = "Option::is_none")]
    pub cp: Option<bool>,
    #[serde(rename = "AP", skip_serializing_if = "Option::is_none")]
    pub ap_uppercase: Option<String>,
    #[serde(rename = "cr", skip_serializing_if = "Option::is_none")]
    pub cr: Option<String>,
    #[serde(rename = "pP", skip_serializing_if = "Option::is_none")]
    pub p_p: Option<bool>,
    #[serde(rename = "si", skip_serializing_if = "Option::is_none")]
    pub si: Option<i64>,
    #[serde(rename = "ss", skip_serializing_if = "Option::is_none")]
    pub ss: Option<i64>,
    #[serde(rename = "rp", skip_serializing_if = "Option::is_none")]
    pub rp: Option<String>,
    #[serde(rename = "V", skip_serializing_if = "Option::is_none")]
    pub v_uppercase: Option<String>,
    #[serde(rename = "pm", skip_serializing_if = "Option::is_none")]
    pub pm: Option<String>,
    #[serde(rename = "gtd", skip_serializing_if = "Option::is_none")]
    pub gtd: Option<i64>,
}

impl OrderTradeUpdateO {
    #[must_use]
    pub fn new() -> OrderTradeUpdateO {
        OrderTradeUpdateO {
            s: None,
            c: None,
            s_uppercase: None,
            o: None,
            f: None,
            q: None,
            p: None,
            ap: None,
            sp: None,
            x: None,
            x_uppercase: None,
            i: None,
            l: None,
            z: None,
            l_uppercase: None,
            n_uppercase: None,
            n: None,
            t_uppercase: None,
            t: None,
            b: None,
            a: None,
            m: None,
            r_uppercase: None,
            wt: None,
            ot: None,
            ps: None,
            cp: None,
            ap_uppercase: None,
            cr: None,
            p_p: None,
            si: None,
            ss: None,
            rp: None,
            v_uppercase: None,
            pm: None,
            gtd: None,
        }
    }
}
