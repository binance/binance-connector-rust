/*
 * Binance Simple Earn REST API
 *
 * OpenAPI Specification for the Binance Simple Earn REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::simple_earn::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFlexibleProductPositionResponseRowsInner {
    #[serde(rename = "totalAmount", skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<String>,
    #[serde(
        rename = "tierAnnualPercentageRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub tier_annual_percentage_rate:
        Option<Box<models::GetFlexibleProductPositionResponseRowsInnerTierAnnualPercentageRate>>,
    #[serde(
        rename = "latestAnnualPercentageRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_annual_percentage_rate: Option<String>,
    #[serde(
        rename = "yesterdayAirdropPercentageRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub yesterday_airdrop_percentage_rate: Option<String>,
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "airDropAsset", skip_serializing_if = "Option::is_none")]
    pub air_drop_asset: Option<String>,
    #[serde(rename = "canRedeem", skip_serializing_if = "Option::is_none")]
    pub can_redeem: Option<bool>,
    #[serde(rename = "collateralAmount", skip_serializing_if = "Option::is_none")]
    pub collateral_amount: Option<String>,
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(
        rename = "yesterdayRealTimeRewards",
        skip_serializing_if = "Option::is_none"
    )]
    pub yesterday_real_time_rewards: Option<String>,
    #[serde(
        rename = "cumulativeBonusRewards",
        skip_serializing_if = "Option::is_none"
    )]
    pub cumulative_bonus_rewards: Option<String>,
    #[serde(
        rename = "cumulativeRealTimeRewards",
        skip_serializing_if = "Option::is_none"
    )]
    pub cumulative_real_time_rewards: Option<String>,
    #[serde(
        rename = "cumulativeTotalRewards",
        skip_serializing_if = "Option::is_none"
    )]
    pub cumulative_total_rewards: Option<String>,
    #[serde(rename = "autoSubscribe", skip_serializing_if = "Option::is_none")]
    pub auto_subscribe: Option<bool>,
}

impl GetFlexibleProductPositionResponseRowsInner {
    #[must_use]
    pub fn new() -> GetFlexibleProductPositionResponseRowsInner {
        GetFlexibleProductPositionResponseRowsInner {
            total_amount: None,
            tier_annual_percentage_rate: None,
            latest_annual_percentage_rate: None,
            yesterday_airdrop_percentage_rate: None,
            asset: None,
            air_drop_asset: None,
            can_redeem: None,
            collateral_amount: None,
            product_id: None,
            yesterday_real_time_rewards: None,
            cumulative_bonus_rewards: None,
            cumulative_real_time_rewards: None,
            cumulative_total_rewards: None,
            auto_subscribe: None,
        }
    }
}
