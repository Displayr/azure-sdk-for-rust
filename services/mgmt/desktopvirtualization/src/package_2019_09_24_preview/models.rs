#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProviderOperationList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderOperation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProviderOperation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<resource_provider_operation::Display>,
}
mod resource_provider_operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "applicationGroupReferences", skip_serializing_if = "Vec::is_empty")]
    pub application_group_references: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspacePatchProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePatchProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "applicationGroupReferences", skip_serializing_if = "Vec::is_empty")]
    pub application_group_references: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationGroup {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: ApplicationGroupProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationGroupProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "hostPoolArmPath")]
    pub host_pool_arm_path: String,
    #[serde(rename = "workspaceArmPath", skip_serializing)]
    pub workspace_arm_path: Option<String>,
    #[serde(rename = "applicationGroupType")]
    pub application_group_type: application_group_properties::ApplicationGroupType,
}
mod application_group_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ApplicationGroupType {
        RemoteApp,
        Desktop,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationGroupPatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationGroupPatchProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationGroupPatchProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationGroupList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationGroup>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostPool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: HostPoolProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostPoolProperties {
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "hostPoolType")]
    pub host_pool_type: host_pool_properties::HostPoolType,
    #[serde(rename = "personalDesktopAssignmentType", skip_serializing_if = "Option::is_none")]
    pub personal_desktop_assignment_type: Option<host_pool_properties::PersonalDesktopAssignmentType>,
    #[serde(rename = "customRdpProperty", skip_serializing_if = "Option::is_none")]
    pub custom_rdp_property: Option<String>,
    #[serde(rename = "maxSessionLimit", skip_serializing_if = "Option::is_none")]
    pub max_session_limit: Option<i64>,
    #[serde(rename = "loadBalancerType")]
    pub load_balancer_type: host_pool_properties::LoadBalancerType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring: Option<i64>,
    #[serde(rename = "validationEnvironment", skip_serializing_if = "Option::is_none")]
    pub validation_environment: Option<bool>,
    #[serde(rename = "registrationInfo", skip_serializing_if = "Option::is_none")]
    pub registration_info: Option<RegistrationInfo>,
    #[serde(rename = "vmTemplate", skip_serializing_if = "Option::is_none")]
    pub vm_template: Option<String>,
    #[serde(rename = "applicationGroupReferences", skip_serializing)]
    pub application_group_references: Vec<String>,
    #[serde(rename = "ssoContext", skip_serializing_if = "Option::is_none")]
    pub sso_context: Option<String>,
    #[serde(rename = "preferredAppGroupType")]
    pub preferred_app_group_type: host_pool_properties::PreferredAppGroupType,
}
mod host_pool_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HostPoolType {
        Personal,
        Shared,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PersonalDesktopAssignmentType {
        Automatic,
        Direct,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LoadBalancerType {
        BreadthFirst,
        DepthFirst,
        Persistent,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PreferredAppGroupType {
        None,
        Desktop,
        RailApplications,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostPoolPatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HostPoolPatchProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostPoolPatchProperties {
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "customRdpProperty", skip_serializing_if = "Option::is_none")]
    pub custom_rdp_property: Option<String>,
    #[serde(rename = "maxSessionLimit", skip_serializing_if = "Option::is_none")]
    pub max_session_limit: Option<i64>,
    #[serde(rename = "personalDesktopAssignmentType", skip_serializing_if = "Option::is_none")]
    pub personal_desktop_assignment_type: Option<host_pool_patch_properties::PersonalDesktopAssignmentType>,
    #[serde(rename = "loadBalancerType", skip_serializing_if = "Option::is_none")]
    pub load_balancer_type: Option<host_pool_patch_properties::LoadBalancerType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring: Option<i64>,
    #[serde(rename = "validationEnvironment", skip_serializing_if = "Option::is_none")]
    pub validation_environment: Option<bool>,
    #[serde(rename = "registrationInfo", skip_serializing_if = "Option::is_none")]
    pub registration_info: Option<RegistrationInfoPatch>,
    #[serde(rename = "ssoContext", skip_serializing_if = "Option::is_none")]
    pub sso_context: Option<String>,
    #[serde(rename = "preferredAppGroupType", skip_serializing_if = "Option::is_none")]
    pub preferred_app_group_type: Option<host_pool_patch_properties::PreferredAppGroupType>,
}
mod host_pool_patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PersonalDesktopAssignmentType {
        Automatic,
        Direct,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LoadBalancerType {
        BreadthFirst,
        DepthFirst,
        Persistent,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PreferredAppGroupType {
        None,
        Desktop,
        RailApplications,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationInfo {
    #[serde(rename = "expirationTime", skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "registrationTokenOperation", skip_serializing_if = "Option::is_none")]
    pub registration_token_operation: Option<registration_info::RegistrationTokenOperation>,
}
mod registration_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RegistrationTokenOperation {
        Delete,
        None,
        Update,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationInfoPatch {
    #[serde(rename = "registrationTokenOperation", skip_serializing_if = "Option::is_none")]
    pub registration_token_operation: Option<registration_info_patch::RegistrationTokenOperation>,
}
mod registration_info_patch {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RegistrationTokenOperation {
        Delete,
        None,
        Update,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendMessage {
    #[serde(rename = "messageTitle", skip_serializing_if = "Option::is_none")]
    pub message_title: Option<String>,
    #[serde(rename = "messageBody", skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostPoolList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HostPool>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: ApplicationProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "filePath", skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "commandLineSetting")]
    pub command_line_setting: application_properties::CommandLineSetting,
    #[serde(rename = "commandLineArguments", skip_serializing_if = "Option::is_none")]
    pub command_line_arguments: Option<String>,
    #[serde(rename = "showInPortal", skip_serializing_if = "Option::is_none")]
    pub show_in_portal: Option<bool>,
    #[serde(rename = "iconPath", skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
    #[serde(rename = "iconIndex", skip_serializing_if = "Option::is_none")]
    pub icon_index: Option<i64>,
    #[serde(rename = "iconHash", skip_serializing)]
    pub icon_hash: Option<String>,
    #[serde(rename = "iconContent", skip_serializing)]
    pub icon_content: Option<String>,
}
mod application_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CommandLineSetting {
        DoNotAllow,
        Allow,
        Require,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationPatchProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPatchProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "filePath", skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "commandLineSetting", skip_serializing_if = "Option::is_none")]
    pub command_line_setting: Option<application_patch_properties::CommandLineSetting>,
    #[serde(rename = "commandLineArguments", skip_serializing_if = "Option::is_none")]
    pub command_line_arguments: Option<String>,
    #[serde(rename = "showInPortal", skip_serializing_if = "Option::is_none")]
    pub show_in_portal: Option<bool>,
    #[serde(rename = "iconPath", skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
    #[serde(rename = "iconIndex", skip_serializing_if = "Option::is_none")]
    pub icon_index: Option<i64>,
}
mod application_patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CommandLineSetting {
        DoNotAllow,
        Allow,
        Require,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Desktop {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DesktopProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DesktopProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "iconHash", skip_serializing)]
    pub icon_hash: Option<String>,
    #[serde(rename = "iconContent", skip_serializing)]
    pub icon_content: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DesktopList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Desktop>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DesktopPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DesktopPatchProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DesktopPatchProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartMenuItemProperties {
    #[serde(rename = "appAlias", skip_serializing_if = "Option::is_none")]
    pub app_alias: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "filePath", skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "commandLineArguments", skip_serializing_if = "Option::is_none")]
    pub command_line_arguments: Option<String>,
    #[serde(rename = "iconPath", skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
    #[serde(rename = "iconIndex", skip_serializing_if = "Option::is_none")]
    pub icon_index: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartMenuItem {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StartMenuItemProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartMenuItemList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StartMenuItem>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionHost {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SessionHostProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionHostProperties {
    #[serde(rename = "lastHeartBeat", skip_serializing_if = "Option::is_none")]
    pub last_heart_beat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<i64>,
    #[serde(rename = "agentVersion", skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "allowNewSession", skip_serializing_if = "Option::is_none")]
    pub allow_new_session: Option<bool>,
    #[serde(rename = "virtualMachineId", skip_serializing)]
    pub virtual_machine_id: Option<String>,
    #[serde(rename = "resourceId", skip_serializing)]
    pub resource_id: Option<String>,
    #[serde(rename = "assignedUser", skip_serializing_if = "Option::is_none")]
    pub assigned_user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<session_host_properties::Status>,
    #[serde(rename = "statusTimestamp", skip_serializing)]
    pub status_timestamp: Option<String>,
    #[serde(rename = "osVersion", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "sxSStackVersion", skip_serializing_if = "Option::is_none")]
    pub sx_s_stack_version: Option<String>,
    #[serde(rename = "updateState", skip_serializing_if = "Option::is_none")]
    pub update_state: Option<session_host_properties::UpdateState>,
    #[serde(rename = "lastUpdateTime", skip_serializing)]
    pub last_update_time: Option<String>,
    #[serde(rename = "updateErrorMessage", skip_serializing_if = "Option::is_none")]
    pub update_error_message: Option<String>,
}
mod session_host_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Available,
        Unavailable,
        Shutdown,
        Disconnected,
        Upgrading,
        UpgradeFailed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UpdateState {
        Initial,
        Pending,
        Started,
        Succeeded,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionHostPatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SessionHostPatchProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionHostPatchProperties {
    #[serde(rename = "allowNewSession", skip_serializing_if = "Option::is_none")]
    pub allow_new_session: Option<bool>,
    #[serde(rename = "assignedUser", skip_serializing_if = "Option::is_none")]
    pub assigned_user: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionHostList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SessionHost>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSession {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserSessionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSessionProperties {
    #[serde(rename = "userPrincipalName", skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[serde(rename = "applicationType", skip_serializing_if = "Option::is_none")]
    pub application_type: Option<user_session_properties::ApplicationType>,
    #[serde(rename = "sessionState", skip_serializing_if = "Option::is_none")]
    pub session_state: Option<user_session_properties::SessionState>,
    #[serde(rename = "activeDirectoryUserName", skip_serializing_if = "Option::is_none")]
    pub active_directory_user_name: Option<String>,
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}
mod user_session_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ApplicationType {
        RemoteApp,
        Desktop,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SessionState {
        Unknown,
        Active,
        Disconnected,
        Pending,
        LogOff,
        UserProfileDiskMounted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSessionList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UserSession>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
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
