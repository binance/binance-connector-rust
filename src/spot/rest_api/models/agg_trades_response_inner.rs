/*
 * Binance Spot REST API
 *
 * OpenAPI Specifications for the Binance Spot REST API
 *
 * API documents:
 * - [Github rest-api documentation file](https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md)
 * - [General API information for rest-api on website](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/general-api-information)
 *
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::spot::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggTradesResponseInner {
    #[serde(rename = "a", skip_serializing_if = "Option::is_none")]
    pub a: Option<i64>,
    #[serde(rename = "p", skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(rename = "f", skip_serializing_if = "Option::is_none")]
    pub f: Option<i64>,
    #[serde(rename = "l", skip_serializing_if = "Option::is_none")]
    pub l: Option<i64>,
    #[serde(rename = "T", skip_serializing_if = "Option::is_none")]
    pub t_uppercase: Option<i64>,
    #[serde(rename = "m", skip_serializing_if = "Option::is_none")]
    pub m: Option<bool>,
    #[serde(rename = "M", skip_serializing_if = "Option::is_none")]
    pub m_uppercase: Option<bool>,
}

impl AggTradesResponseInner {
    #[must_use]
    pub fn new() -> AggTradesResponseInner {
        AggTradesResponseInner {
            a: None,
            p: None,
            q: None,
            f: None,
            l: None,
            t_uppercase: None,
            m: None,
            m_uppercase: None,
        }
    }
}
