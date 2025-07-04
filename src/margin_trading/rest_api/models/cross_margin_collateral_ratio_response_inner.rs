/*
 * Binance Margin Trading REST API
 *
 * OpenAPI Specification for the Binance Margin Trading REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::margin_trading::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CrossMarginCollateralRatioResponseInner {
    #[serde(rename = "collaterals", skip_serializing_if = "Option::is_none")]
    pub collaterals: Option<Vec<models::CrossMarginCollateralRatioResponseInnerCollateralsInner>>,
    #[serde(rename = "assetNames", skip_serializing_if = "Option::is_none")]
    pub asset_names: Option<Vec<String>>,
}

impl CrossMarginCollateralRatioResponseInner {
    #[must_use]
    pub fn new() -> CrossMarginCollateralRatioResponseInner {
        CrossMarginCollateralRatioResponseInner {
            collaterals: None,
            asset_names: None,
        }
    }
}
