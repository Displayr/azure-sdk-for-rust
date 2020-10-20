#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolutionProperties {
    #[serde(rename = "workspaceResourceId")]
    pub workspace_resource_id: String,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "containedResources", skip_serializing_if = "Vec::is_empty")]
    pub contained_resources: Vec<String>,
    #[serde(rename = "referencedResources", skip_serializing_if = "Vec::is_empty")]
    pub referenced_resources: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementAssociationProperties {
    #[serde(rename = "applicationId")]
    pub application_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementConfigurationProperties {
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "parentResourceType")]
    pub parent_resource_type: String,
    pub parameters: Vec<ArmTemplateParameter>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    pub template: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolutionPropertiesList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Solution>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementAssociationPropertiesList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagementAssociation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementConfigurationPropertiesList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagementConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolutionPlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "promotionCode", skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Solution {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<SolutionPlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SolutionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolutionPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementAssociation {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementAssociationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementConfiguration {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementConfigurationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmTemplateParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeMessageError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<code_message_error::Error>,
}
mod code_message_error {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Error {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
}
