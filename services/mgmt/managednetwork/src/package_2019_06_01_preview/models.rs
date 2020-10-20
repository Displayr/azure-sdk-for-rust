#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedNetwork {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedNetworkProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedNetworkProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity: Option<ConnectivityCollection>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedNetworkUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedNetworkListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedNetwork>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scope {
    #[serde(rename = "managementGroups", skip_serializing_if = "Vec::is_empty")]
    pub management_groups: Vec<ResourceId>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<ResourceId>,
    #[serde(rename = "virtualNetworks", skip_serializing_if = "Vec::is_empty")]
    pub virtual_networks: Vec<ResourceId>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<ResourceId>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeAssignment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScopeAssignmentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeAssignmentListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScopeAssignment>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeAssignmentProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[serde(rename = "assignedManagedNetwork", skip_serializing_if = "Option::is_none")]
    pub assigned_managed_network: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectivityCollection {
    #[serde(skip_serializing)]
    pub groups: Vec<ManagedNetworkGroup>,
    #[serde(skip_serializing)]
    pub peerings: Vec<ManagedNetworkPeeringPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedNetworkGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedNetworkGroupProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<managed_network_group::Kind>,
}
mod managed_network_group {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Connectivity,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedNetworkGroupProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[serde(rename = "managementGroups", skip_serializing_if = "Vec::is_empty")]
    pub management_groups: Vec<ResourceId>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<ResourceId>,
    #[serde(rename = "virtualNetworks", skip_serializing_if = "Vec::is_empty")]
    pub virtual_networks: Vec<ResourceId>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<ResourceId>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedNetworkGroupListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedNetworkGroup>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedNetworkPeeringPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedNetworkPeeringPolicyProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedNetworkPeeringPolicyProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[serde(rename = "type")]
    pub type_: managed_network_peering_policy_properties::Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub: Option<ResourceId>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub spokes: Vec<ResourceId>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mesh: Vec<ResourceId>,
}
mod managed_network_peering_policy_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        HubAndSpokeTopology,
        MeshTopology,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HubAndSpokePeeringPolicyProperties {
    #[serde(flatten)]
    pub managed_network_peering_policy_properties: ManagedNetworkPeeringPolicyProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub: Option<ResourceId>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub spokes: Vec<ResourceId>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeshPeeringPolicyProperties {
    #[serde(flatten)]
    pub managed_network_peering_policy_properties: ManagedNetworkPeeringPolicyProperties,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mesh: Vec<ResourceId>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedNetworkPeeringPolicyListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedNetworkPeeringPolicy>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
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
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<resource_properties::ProvisioningState>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
}
mod resource_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Updating,
        Deleting,
        Failed,
        Succeeded,
    }
}
