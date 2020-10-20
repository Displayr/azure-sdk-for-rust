#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<serde_json::Value>,
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
pub struct MapsAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MapsAccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsAccountCreateParameters {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub sku: Sku,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsAccountUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsAccounts {
    #[serde(skip_serializing)]
    pub value: Vec<MapsAccount>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
    #[serde(skip_serializing)]
    pub tier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsAccountsMoveRequest {
    #[serde(rename = "targetResourceGroup")]
    pub target_resource_group: String,
    #[serde(rename = "resourceIds")]
    pub resource_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsKeySpecification {
    #[serde(rename = "keyType")]
    pub key_type: maps_key_specification::KeyType,
}
mod maps_key_specification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "secondary")]
        Secondary,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsAccountKeys {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "primaryKey", skip_serializing)]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", skip_serializing)]
    pub secondary_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsOperations {
    #[serde(skip_serializing)]
    pub value: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsAccountProperties {
    #[serde(rename = "x-ms-client-id", skip_serializing_if = "Option::is_none")]
    pub x_ms_client_id: Option<String>,
}
