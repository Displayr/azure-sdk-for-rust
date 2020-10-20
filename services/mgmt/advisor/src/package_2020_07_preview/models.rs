#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ArmErrorResponseBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmErrorResponseBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntityListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplayInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScoreEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(rename = "consumptionUnits", skip_serializing_if = "Option::is_none")]
    pub consumption_units: Option<f64>,
    #[serde(rename = "impactedResourceCount", skip_serializing_if = "Option::is_none")]
    pub impacted_resource_count: Option<f64>,
    #[serde(rename = "potentialScoreIncrease", skip_serializing_if = "Option::is_none")]
    pub potential_score_increase: Option<f64>,
}
pub type TimeSeriesEntity = Vec<serde_json::Value>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdvisorScoreEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<advisor_score_entity::Properties>,
}
mod advisor_score_entity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "lastRefreshedScore", skip_serializing_if = "Option::is_none")]
        pub last_refreshed_score: Option<ScoreEntity>,
        #[serde(rename = "timeSeries", skip_serializing_if = "Option::is_none")]
        pub time_series: Option<TimeSeriesEntity>,
    }
}
