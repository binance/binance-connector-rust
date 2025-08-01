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
pub struct GetDetailOnSubAccountsFuturesAccountV2Response {
    #[serde(rename = "futureAccountResp", skip_serializing_if = "Option::is_none")]
    pub future_account_resp:
        Option<Box<models::GetDetailOnSubAccountsFuturesAccountV2ResponseFutureAccountResp>>,
    #[serde(
        rename = "deliveryAccountResp",
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_account_resp:
        Option<Box<models::GetDetailOnSubAccountsFuturesAccountV2ResponseDeliveryAccountResp>>,
}

impl GetDetailOnSubAccountsFuturesAccountV2Response {
    #[must_use]
    pub fn new() -> GetDetailOnSubAccountsFuturesAccountV2Response {
        GetDetailOnSubAccountsFuturesAccountV2Response {
            future_account_resp: None,
            delivery_account_resp: None,
        }
    }
}
