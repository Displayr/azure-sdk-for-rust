#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeStoreAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<EncryptionIdentity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeStoreAccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeStoreAccountBasic {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeStoreAccountPropertiesBasic>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeStoreAccountProperties {
    #[serde(flatten)]
    pub data_lake_store_account_properties_basic: DataLakeStoreAccountPropertiesBasic,
    #[serde(rename = "defaultGroup", skip_serializing)]
    pub default_group: Option<String>,
    #[serde(rename = "encryptionConfig", skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    #[serde(rename = "encryptionState", skip_serializing)]
    pub encryption_state: Option<data_lake_store_account_properties::EncryptionState>,
    #[serde(rename = "encryptionProvisioningState", skip_serializing)]
    pub encryption_provisioning_state: Option<data_lake_store_account_properties::EncryptionProvisioningState>,
    #[serde(rename = "firewallRules", skip_serializing)]
    pub firewall_rules: Vec<FirewallRule>,
    #[serde(rename = "virtualNetworkRules", skip_serializing)]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
    #[serde(rename = "firewallState", skip_serializing)]
    pub firewall_state: Option<data_lake_store_account_properties::FirewallState>,
    #[serde(rename = "firewallAllowAzureIps", skip_serializing)]
    pub firewall_allow_azure_ips: Option<data_lake_store_account_properties::FirewallAllowAzureIps>,
    #[serde(rename = "trustedIdProviders", skip_serializing)]
    pub trusted_id_providers: Vec<TrustedIdProvider>,
    #[serde(rename = "trustedIdProviderState", skip_serializing)]
    pub trusted_id_provider_state: Option<data_lake_store_account_properties::TrustedIdProviderState>,
    #[serde(rename = "newTier", skip_serializing)]
    pub new_tier: Option<data_lake_store_account_properties::NewTier>,
    #[serde(rename = "currentTier", skip_serializing)]
    pub current_tier: Option<data_lake_store_account_properties::CurrentTier>,
}
mod data_lake_store_account_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionState {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionProvisioningState {
        Creating,
        Succeeded,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallState {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallAllowAzureIps {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TrustedIdProviderState {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTier {
        Consumption,
        #[serde(rename = "Commitment_1TB")]
        Commitment1tb,
        #[serde(rename = "Commitment_10TB")]
        Commitment10tb,
        #[serde(rename = "Commitment_100TB")]
        Commitment100tb,
        #[serde(rename = "Commitment_500TB")]
        Commitment500tb,
        #[serde(rename = "Commitment_1PB")]
        Commitment1pb,
        #[serde(rename = "Commitment_5PB")]
        Commitment5pb,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CurrentTier {
        Consumption,
        #[serde(rename = "Commitment_1TB")]
        Commitment1tb,
        #[serde(rename = "Commitment_10TB")]
        Commitment10tb,
        #[serde(rename = "Commitment_100TB")]
        Commitment100tb,
        #[serde(rename = "Commitment_500TB")]
        Commitment500tb,
        #[serde(rename = "Commitment_1PB")]
        Commitment1pb,
        #[serde(rename = "Commitment_5PB")]
        Commitment5pb,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeStoreAccountPropertiesBasic {
    #[serde(rename = "accountId", skip_serializing)]
    pub account_id: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<data_lake_store_account_properties_basic::ProvisioningState>,
    #[serde(skip_serializing)]
    pub state: Option<data_lake_store_account_properties_basic::State>,
    #[serde(rename = "creationTime", skip_serializing)]
    pub creation_time: Option<String>,
    #[serde(rename = "lastModifiedTime", skip_serializing)]
    pub last_modified_time: Option<String>,
    #[serde(skip_serializing)]
    pub endpoint: Option<String>,
}
mod data_lake_store_account_properties_basic {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Failed,
        Creating,
        Running,
        Succeeded,
        Patching,
        Suspending,
        Resuming,
        Deleting,
        Deleted,
        Undeleting,
        Canceled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Active,
        Suspended,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeStoreAccountListResult {
    #[serde(skip_serializing)]
    pub value: Vec<DataLakeStoreAccountBasic>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionIdentity {
    #[serde(rename = "type")]
    pub type_: encryption_identity::Type,
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
}
mod encryption_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionConfig {
    #[serde(rename = "type")]
    pub type_: encryption_config::Type,
    #[serde(rename = "keyVaultMetaInfo", skip_serializing_if = "Option::is_none")]
    pub key_vault_meta_info: Option<KeyVaultMetaInfo>,
}
mod encryption_config {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        UserManaged,
        ServiceManaged,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultMetaInfo {
    #[serde(rename = "keyVaultResourceId")]
    pub key_vault_resource_id: String,
    #[serde(rename = "encryptionKeyName")]
    pub encryption_key_name: String,
    #[serde(rename = "encryptionKeyVersion")]
    pub encryption_key_version: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<FirewallRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleProperties {
    #[serde(rename = "startIpAddress", skip_serializing)]
    pub start_ip_address: Option<String>,
    #[serde(rename = "endIpAddress", skip_serializing)]
    pub end_ip_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleListResult {
    #[serde(skip_serializing)]
    pub value: Vec<FirewallRule>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRuleProperties {
    #[serde(rename = "subnetId", skip_serializing)]
    pub subnet_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRuleListResult {
    #[serde(skip_serializing)]
    pub value: Vec<VirtualNetworkRule>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrustedIdProvider {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TrustedIdProviderProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrustedIdProviderProperties {
    #[serde(rename = "idProvider", skip_serializing)]
    pub id_provider: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrustedIdProviderListResult {
    #[serde(skip_serializing)]
    pub value: Vec<TrustedIdProvider>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(skip_serializing)]
    pub origin: Option<operation::Origin>,
}
mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[serde(skip_serializing)]
    pub provider: Option<String>,
    #[serde(skip_serializing)]
    pub resource: Option<String>,
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilityInformation {
    #[serde(rename = "subscriptionId", skip_serializing)]
    pub subscription_id: Option<String>,
    #[serde(skip_serializing)]
    pub state: Option<capability_information::State>,
    #[serde(rename = "maxAccountCount", skip_serializing)]
    pub max_account_count: Option<i32>,
    #[serde(rename = "accountCount", skip_serializing)]
    pub account_count: Option<i32>,
    #[serde(rename = "migrationState", skip_serializing)]
    pub migration_state: Option<bool>,
}
mod capability_information {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Registered,
        Suspended,
        Deleted,
        Unregistered,
        Warned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageName {
    #[serde(skip_serializing)]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", skip_serializing)]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[serde(skip_serializing)]
    pub unit: Option<usage::Unit>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "currentValue", skip_serializing)]
    pub current_value: Option<i32>,
    #[serde(skip_serializing)]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageName>,
}
mod usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountsPerSecond,
        BytesPerSecond,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityInformation {
    #[serde(rename = "nameAvailable", skip_serializing)]
    pub name_available: Option<bool>,
    #[serde(skip_serializing)]
    pub reason: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateDataLakeStoreAccountParameters {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<EncryptionIdentity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CreateDataLakeStoreAccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateDataLakeStoreAccountProperties {
    #[serde(rename = "defaultGroup", skip_serializing_if = "Option::is_none")]
    pub default_group: Option<String>,
    #[serde(rename = "encryptionConfig", skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    #[serde(rename = "encryptionState", skip_serializing_if = "Option::is_none")]
    pub encryption_state: Option<create_data_lake_store_account_properties::EncryptionState>,
    #[serde(rename = "firewallRules", skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<CreateFirewallRuleWithAccountParameters>,
    #[serde(rename = "virtualNetworkRules", skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<CreateVirtualNetworkRuleWithAccountParameters>,
    #[serde(rename = "firewallState", skip_serializing_if = "Option::is_none")]
    pub firewall_state: Option<create_data_lake_store_account_properties::FirewallState>,
    #[serde(rename = "firewallAllowAzureIps", skip_serializing_if = "Option::is_none")]
    pub firewall_allow_azure_ips: Option<create_data_lake_store_account_properties::FirewallAllowAzureIps>,
    #[serde(rename = "trustedIdProviders", skip_serializing_if = "Vec::is_empty")]
    pub trusted_id_providers: Vec<CreateTrustedIdProviderWithAccountParameters>,
    #[serde(rename = "trustedIdProviderState", skip_serializing_if = "Option::is_none")]
    pub trusted_id_provider_state: Option<create_data_lake_store_account_properties::TrustedIdProviderState>,
    #[serde(rename = "newTier", skip_serializing_if = "Option::is_none")]
    pub new_tier: Option<create_data_lake_store_account_properties::NewTier>,
}
mod create_data_lake_store_account_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionState {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallState {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallAllowAzureIps {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TrustedIdProviderState {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTier {
        Consumption,
        #[serde(rename = "Commitment_1TB")]
        Commitment1tb,
        #[serde(rename = "Commitment_10TB")]
        Commitment10tb,
        #[serde(rename = "Commitment_100TB")]
        Commitment100tb,
        #[serde(rename = "Commitment_500TB")]
        Commitment500tb,
        #[serde(rename = "Commitment_1PB")]
        Commitment1pb,
        #[serde(rename = "Commitment_5PB")]
        Commitment5pb,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDataLakeStoreAccountParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateDataLakeStoreAccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDataLakeStoreAccountProperties {
    #[serde(rename = "defaultGroup", skip_serializing_if = "Option::is_none")]
    pub default_group: Option<String>,
    #[serde(rename = "encryptionConfig", skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<UpdateEncryptionConfig>,
    #[serde(rename = "firewallRules", skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<UpdateFirewallRuleWithAccountParameters>,
    #[serde(rename = "virtualNetworkRules", skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<UpdateVirtualNetworkRuleWithAccountParameters>,
    #[serde(rename = "firewallState", skip_serializing_if = "Option::is_none")]
    pub firewall_state: Option<update_data_lake_store_account_properties::FirewallState>,
    #[serde(rename = "firewallAllowAzureIps", skip_serializing_if = "Option::is_none")]
    pub firewall_allow_azure_ips: Option<update_data_lake_store_account_properties::FirewallAllowAzureIps>,
    #[serde(rename = "trustedIdProviders", skip_serializing_if = "Vec::is_empty")]
    pub trusted_id_providers: Vec<UpdateTrustedIdProviderWithAccountParameters>,
    #[serde(rename = "trustedIdProviderState", skip_serializing_if = "Option::is_none")]
    pub trusted_id_provider_state: Option<update_data_lake_store_account_properties::TrustedIdProviderState>,
    #[serde(rename = "newTier", skip_serializing_if = "Option::is_none")]
    pub new_tier: Option<update_data_lake_store_account_properties::NewTier>,
}
mod update_data_lake_store_account_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallState {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallAllowAzureIps {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TrustedIdProviderState {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTier {
        Consumption,
        #[serde(rename = "Commitment_1TB")]
        Commitment1tb,
        #[serde(rename = "Commitment_10TB")]
        Commitment10tb,
        #[serde(rename = "Commitment_100TB")]
        Commitment100tb,
        #[serde(rename = "Commitment_500TB")]
        Commitment500tb,
        #[serde(rename = "Commitment_1PB")]
        Commitment1pb,
        #[serde(rename = "Commitment_5PB")]
        Commitment5pb,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateEncryptionConfig {
    #[serde(rename = "keyVaultMetaInfo", skip_serializing_if = "Option::is_none")]
    pub key_vault_meta_info: Option<UpdateKeyVaultMetaInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateKeyVaultMetaInfo {
    #[serde(rename = "encryptionKeyVersion", skip_serializing_if = "Option::is_none")]
    pub encryption_key_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateFirewallRuleParameters {
    pub properties: CreateOrUpdateFirewallRuleProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFirewallRuleWithAccountParameters {
    pub name: String,
    pub properties: CreateOrUpdateFirewallRuleProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateFirewallRuleProperties {
    #[serde(rename = "startIpAddress")]
    pub start_ip_address: String,
    #[serde(rename = "endIpAddress")]
    pub end_ip_address: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateFirewallRuleParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateFirewallRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateFirewallRuleWithAccountParameters {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateFirewallRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateFirewallRuleProperties {
    #[serde(rename = "startIpAddress", skip_serializing_if = "Option::is_none")]
    pub start_ip_address: Option<String>,
    #[serde(rename = "endIpAddress", skip_serializing_if = "Option::is_none")]
    pub end_ip_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateVirtualNetworkRuleParameters {
    pub properties: CreateOrUpdateVirtualNetworkRuleProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateVirtualNetworkRuleWithAccountParameters {
    pub name: String,
    pub properties: CreateOrUpdateVirtualNetworkRuleProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateVirtualNetworkRuleProperties {
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateVirtualNetworkRuleParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateVirtualNetworkRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateVirtualNetworkRuleWithAccountParameters {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateVirtualNetworkRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateVirtualNetworkRuleProperties {
    #[serde(rename = "subnetId", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateTrustedIdProviderParameters {
    pub properties: CreateOrUpdateTrustedIdProviderProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTrustedIdProviderWithAccountParameters {
    pub name: String,
    pub properties: CreateOrUpdateTrustedIdProviderProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateTrustedIdProviderProperties {
    #[serde(rename = "idProvider")]
    pub id_provider: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateTrustedIdProviderParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateTrustedIdProviderProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateTrustedIdProviderWithAccountParameters {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateTrustedIdProviderProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateTrustedIdProviderProperties {
    #[serde(rename = "idProvider", skip_serializing_if = "Option::is_none")]
    pub id_provider: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: check_name_availability_parameters::Type,
}
mod check_name_availability_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.DataLakeStore/accounts")]
        Microsoft_DataLakeStoreAccounts,
    }
}
