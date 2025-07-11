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
pub struct QueryManagedSubAccountSnapshotResponse {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(rename = "msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(rename = "snapshotVos", skip_serializing_if = "Option::is_none")]
    pub snapshot_vos: Option<Vec<models::QueryManagedSubAccountSnapshotResponseSnapshotVosInner>>,
}

impl QueryManagedSubAccountSnapshotResponse {
    #[must_use]
    pub fn new() -> QueryManagedSubAccountSnapshotResponse {
        QueryManagedSubAccountSnapshotResponse {
            code: None,
            msg: None,
            snapshot_vos: None,
        }
    }
}
