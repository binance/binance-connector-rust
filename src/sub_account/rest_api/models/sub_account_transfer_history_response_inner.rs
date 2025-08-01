/*
 * Binance Sub Account REST API
 *
 * OpenAPI Specification for the Binance Sub Account REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::sub_account::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubAccountTransferHistoryResponseInner {
    #[serde(rename = "counterParty", skip_serializing_if = "Option::is_none")]
    pub counter_party: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(rename = "fromAccountType", skip_serializing_if = "Option::is_none")]
    pub from_account_type: Option<String>,
    #[serde(rename = "toAccountType", skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "tranId", skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<i64>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

impl SubAccountTransferHistoryResponseInner {
    #[must_use]
    pub fn new() -> SubAccountTransferHistoryResponseInner {
        SubAccountTransferHistoryResponseInner {
            counter_party: None,
            email: None,
            r#type: None,
            asset: None,
            qty: None,
            from_account_type: None,
            to_account_type: None,
            status: None,
            tran_id: None,
            time: None,
        }
    }
}
