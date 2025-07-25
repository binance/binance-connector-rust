/*
 * Binance Wallet REST API
 *
 * OpenAPI Specification for the Binance Wallet REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::wallet::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DailyAccountSnapshotResponseSnapshotVosInnerData {
    #[serde(rename = "balances", skip_serializing_if = "Option::is_none")]
    pub balances:
        Option<Vec<models::DailyAccountSnapshotResponseSnapshotVosInnerDataBalancesInner>>,
    #[serde(rename = "totalAssetOfBtc", skip_serializing_if = "Option::is_none")]
    pub total_asset_of_btc: Option<String>,
    #[serde(rename = "marginLevel", skip_serializing_if = "Option::is_none")]
    pub margin_level: Option<String>,
    #[serde(
        rename = "totalLiabilityOfBtc",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_liability_of_btc: Option<String>,
    #[serde(rename = "totalNetAssetOfBtc", skip_serializing_if = "Option::is_none")]
    pub total_net_asset_of_btc: Option<String>,
    #[serde(rename = "userAssets", skip_serializing_if = "Option::is_none")]
    pub user_assets:
        Option<Vec<models::DailyAccountSnapshotResponseSnapshotVosInnerDataUserAssetsInner>>,
    #[serde(rename = "assets", skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<models::DailyAccountSnapshotResponseSnapshotVosInnerDataAssetsInner>>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position:
        Option<Vec<models::DailyAccountSnapshotResponseSnapshotVosInnerDataPositionInner>>,
}

impl DailyAccountSnapshotResponseSnapshotVosInnerData {
    #[must_use]
    pub fn new() -> DailyAccountSnapshotResponseSnapshotVosInnerData {
        DailyAccountSnapshotResponseSnapshotVosInnerData {
            balances: None,
            total_asset_of_btc: None,
            margin_level: None,
            total_liability_of_btc: None,
            total_net_asset_of_btc: None,
            user_assets: None,
            assets: None,
            position: None,
        }
    }
}
