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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainService {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DomainServiceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainServiceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DomainService>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainServiceProperties {
    #[serde(skip_serializing)]
    pub version: Option<i64>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "domainName", skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "deploymentId", skip_serializing)]
    pub deployment_id: Option<String>,
    #[serde(rename = "syncOwner", skip_serializing)]
    pub sync_owner: Option<String>,
    #[serde(rename = "replicaSets", skip_serializing_if = "Vec::is_empty")]
    pub replica_sets: Vec<ReplicaSet>,
    #[serde(rename = "ldapsSettings", skip_serializing_if = "Option::is_none")]
    pub ldaps_settings: Option<LdapsSettings>,
    #[serde(rename = "domainSecuritySettings", skip_serializing_if = "Option::is_none")]
    pub domain_security_settings: Option<DomainSecuritySettings>,
    #[serde(rename = "filteredSync", skip_serializing_if = "Option::is_none")]
    pub filtered_sync: Option<domain_service_properties::FilteredSync>,
    #[serde(rename = "notificationSettings", skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<NotificationSettings>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
}
mod domain_service_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FilteredSync {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicaSet {
    #[serde(rename = "replicaSetId", skip_serializing)]
    pub replica_set_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "vnetSiteId", skip_serializing)]
    pub vnet_site_id: Option<String>,
    #[serde(rename = "subnetId", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "domainControllerIpAddress", skip_serializing)]
    pub domain_controller_ip_address: Vec<String>,
    #[serde(rename = "externalAccessIpAddress", skip_serializing)]
    pub external_access_ip_address: Option<String>,
    #[serde(rename = "serviceStatus", skip_serializing)]
    pub service_status: Option<String>,
    #[serde(rename = "healthLastEvaluated", skip_serializing)]
    pub health_last_evaluated: Option<String>,
    #[serde(rename = "healthMonitors", skip_serializing)]
    pub health_monitors: Vec<HealthMonitor>,
    #[serde(rename = "healthAlerts", skip_serializing)]
    pub health_alerts: Vec<HealthAlert>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LdapsSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldaps: Option<ldaps_settings::Ldaps>,
    #[serde(rename = "pfxCertificate", skip_serializing_if = "Option::is_none")]
    pub pfx_certificate: Option<String>,
    #[serde(rename = "pfxCertificatePassword", skip_serializing_if = "Option::is_none")]
    pub pfx_certificate_password: Option<String>,
    #[serde(rename = "publicCertificate", skip_serializing)]
    pub public_certificate: Option<String>,
    #[serde(rename = "certificateThumbprint", skip_serializing)]
    pub certificate_thumbprint: Option<String>,
    #[serde(rename = "certificateNotAfter", skip_serializing)]
    pub certificate_not_after: Option<String>,
    #[serde(rename = "externalAccess", skip_serializing_if = "Option::is_none")]
    pub external_access: Option<ldaps_settings::ExternalAccess>,
}
mod ldaps_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Ldaps {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExternalAccess {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthMonitor {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub details: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthAlert {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub issue: Option<String>,
    #[serde(skip_serializing)]
    pub severity: Option<String>,
    #[serde(skip_serializing)]
    pub raised: Option<String>,
    #[serde(rename = "lastDetected", skip_serializing)]
    pub last_detected: Option<String>,
    #[serde(rename = "resolutionUri", skip_serializing)]
    pub resolution_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationSettings {
    #[serde(rename = "notifyGlobalAdmins", skip_serializing_if = "Option::is_none")]
    pub notify_global_admins: Option<notification_settings::NotifyGlobalAdmins>,
    #[serde(rename = "notifyDcAdmins", skip_serializing_if = "Option::is_none")]
    pub notify_dc_admins: Option<notification_settings::NotifyDcAdmins>,
    #[serde(rename = "additionalRecipients", skip_serializing_if = "Vec::is_empty")]
    pub additional_recipients: Vec<String>,
}
mod notification_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NotifyGlobalAdmins {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NotifyDcAdmins {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainSecuritySettings {
    #[serde(rename = "ntlmV1", skip_serializing_if = "Option::is_none")]
    pub ntlm_v1: Option<domain_security_settings::NtlmV1>,
    #[serde(rename = "tlsV1", skip_serializing_if = "Option::is_none")]
    pub tls_v1: Option<domain_security_settings::TlsV1>,
    #[serde(rename = "syncNtlmPasswords", skip_serializing_if = "Option::is_none")]
    pub sync_ntlm_passwords: Option<domain_security_settings::SyncNtlmPasswords>,
}
mod domain_security_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NtlmV1 {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TlsV1 {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SyncNtlmPasswords {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntityListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
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
pub struct OuContainerListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OuContainer>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OuContainer {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<OuContainerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OuContainerProperties {
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "domainName", skip_serializing)]
    pub domain_name: Option<String>,
    #[serde(rename = "deploymentId", skip_serializing)]
    pub deployment_id: Option<String>,
    #[serde(rename = "containerId", skip_serializing)]
    pub container_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<ContainerAccount>,
    #[serde(rename = "serviceStatus", skip_serializing)]
    pub service_status: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerAccount {
    #[serde(rename = "accountName", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
