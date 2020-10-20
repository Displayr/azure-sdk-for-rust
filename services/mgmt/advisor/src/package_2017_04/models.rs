#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataEntityListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MetadataEntity>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetadataEntityProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataEntityProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "dependsOn", skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[serde(rename = "applicableScenarios", skip_serializing_if = "Vec::is_empty")]
    pub applicable_scenarios: Vec<String>,
    #[serde(rename = "supportedValues", skip_serializing_if = "Vec::is_empty")]
    pub supported_values: Vec<MetadataSupportedValueDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataSupportedValueDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConfigData>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<config_data::Properties>,
}
mod config_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exclude: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub low_cpu_threshold: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmErrorResponseBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRecommendationBaseListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceRecommendationBase>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRecommendationBase {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecommendationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendationProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<recommendation_properties::Category>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<recommendation_properties::Impact>,
    #[serde(rename = "impactedField", skip_serializing_if = "Option::is_none")]
    pub impacted_field: Option<String>,
    #[serde(rename = "impactedValue", skip_serializing_if = "Option::is_none")]
    pub impacted_value: Option<String>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "recommendationTypeId", skip_serializing_if = "Option::is_none")]
    pub recommendation_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk: Option<recommendation_properties::Risk>,
    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<ShortDescription>,
    #[serde(rename = "suppressionIds", skip_serializing_if = "Vec::is_empty")]
    pub suppression_ids: Vec<String>,
    #[serde(rename = "extendedProperties", skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<serde_json::Value>,
}
mod recommendation_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Category {
        HighAvailability,
        Security,
        Performance,
        Cost,
        OperationalExcellence,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Impact {
        High,
        Medium,
        Low,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Risk {
        Error,
        Warning,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution: Option<String>,
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
pub struct SuppressionContract {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SuppressionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuppressionProperties {
    #[serde(rename = "suppressionId", skip_serializing_if = "Option::is_none")]
    pub suppression_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuppressionContractListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SuppressionContract>,
}
