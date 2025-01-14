#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingList {
    pub value: Vec<Settings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    pub kind: settings::Kind,
}
pub mod settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        EyesOn,
        EntityAnalytics,
        Ueba,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EyesOn {
    #[serde(flatten)]
    pub settings: Settings,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EyesOnSettingsProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EyesOnSettingsProperties {
    #[serde(rename = "isEnabled", skip_serializing)]
    pub is_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityAnalytics {
    #[serde(flatten)]
    pub settings: Settings,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EntityAnalyticsProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityAnalyticsProperties {
    #[serde(rename = "isEnabled", skip_serializing)]
    pub is_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ueba {
    #[serde(flatten)]
    pub settings: Settings,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UebaProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UebaProperties {
    #[serde(rename = "dataSources", default, skip_serializing_if = "Vec::is_empty")]
    pub data_sources: Vec<UebaDataSources>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UebaDataSources {
    AuditLogs,
    AzureActivity,
    SecurityEvent,
    SigninLogs,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnboardingRequest {
    #[serde(rename = "customerManagedKey", default, skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<bool>,
    #[serde(rename = "eyesOn", default, skip_serializing_if = "Option::is_none")]
    pub eyes_on: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(rename = "entityAnalytics", default, skip_serializing_if = "Option::is_none")]
    pub entity_analytics: Option<EntityAnalyticsOnboardingParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<sku::Name>,
    #[serde(rename = "capacityReservationLevel", default, skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_level: Option<i32>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "PerGB")]
        PerGb,
        CapacityReservation,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityAnalyticsOnboardingParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "uebaDataSources", default, skip_serializing_if = "Option::is_none")]
    pub ueba_data_sources: Option<UebaProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentinelOnboardingStateProperties {
    #[serde(rename = "customerManagedKey", default, skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentinelOnboardingState {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SentinelOnboardingStateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentinelOnboardingStatesList {
    pub value: Vec<SentinelOnboardingState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AadDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AadDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AadDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(flatten)]
    pub data_connector_with_alerts_properties: DataConnectorWithAlertsProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsDataTypeOfDataConnector {
    pub alerts: DataConnectorDataTypeCommon,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MstiDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MstiDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MstiDataConnectorDataTypes {
    #[serde(rename = "bingSafetyPhishingURL")]
    pub bing_safety_phishing_url: msti_data_connector_data_types::BingSafetyPhishingUrl,
    #[serde(rename = "microsoftEmergingThreatFeed")]
    pub microsoft_emerging_threat_feed: msti_data_connector_data_types::MicrosoftEmergingThreatFeed,
}
pub mod msti_data_connector_data_types {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct BingSafetyPhishingUrl {
        #[serde(flatten)]
        pub data_connector_data_type_common: DataConnectorDataTypeCommon,
        #[serde(rename = "lookbackPeriod")]
        pub lookback_period: String,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct MicrosoftEmergingThreatFeed {
        #[serde(flatten)]
        pub data_connector_data_type_common: DataConnectorDataTypeCommon,
        #[serde(rename = "lookbackPeriod")]
        pub lookback_period: String,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MstiDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(rename = "dataTypes")]
    pub data_types: MstiDataConnectorDataTypes,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MtpDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MtpDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MtpDataConnectorDataTypes {
    pub incidents: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MtpDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(rename = "dataTypes")]
    pub data_types: MtpDataConnectorDataTypes,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AatpDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AatpDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AatpDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(flatten)]
    pub data_connector_with_alerts_properties: DataConnectorWithAlertsProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AscDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AscDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AscDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_with_alerts_properties: DataConnectorWithAlertsProperties,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudTrailDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AwsCloudTrailDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudTrailDataConnectorDataTypes {
    pub logs: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudTrailDataConnectorProperties {
    #[serde(rename = "awsRoleArn", default, skip_serializing_if = "Option::is_none")]
    pub aws_role_arn: Option<String>,
    #[serde(rename = "dataTypes")]
    pub data_types: AwsCloudTrailDataConnectorDataTypes,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnector {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    pub kind: DataConnectorKind,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorDataTypeCommon {
    pub state: data_connector_data_type_common::State,
}
pub mod data_connector_data_type_common {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorWithAlertsProperties {
    #[serde(rename = "dataTypes", default, skip_serializing_if = "Option::is_none")]
    pub data_types: Option<AlertsDataTypeOfDataConnector>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DataConnectorKind {
    AzureActiveDirectory,
    AzureSecurityCenter,
    MicrosoftCloudAppSecurity,
    ThreatIntelligence,
    ThreatIntelligenceTaxii,
    Office365,
    #[serde(rename = "OfficeATP")]
    OfficeAtp,
    AmazonWebServicesCloudTrail,
    AzureAdvancedThreatProtection,
    MicrosoftDefenderAdvancedThreatProtection,
    Dynamics365,
    MicrosoftThreatProtection,
    MicrosoftThreatIntelligence,
    #[serde(rename = "GenericUI")]
    GenericUi,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorList {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    pub value: Vec<DataConnector>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorTenantId {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct McasDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<McasDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct McasDataConnectorDataTypes {
    #[serde(flatten)]
    pub alerts_data_type_of_data_connector: AlertsDataTypeOfDataConnector,
    #[serde(rename = "discoveryLogs", default, skip_serializing_if = "Option::is_none")]
    pub discovery_logs: Option<DataConnectorDataTypeCommon>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct McasDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(rename = "dataTypes")]
    pub data_types: McasDataConnectorDataTypes,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dynamics365DataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Dynamics365DataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dynamics365DataConnectorDataTypes {
    #[serde(rename = "dynamics365CdsActivities")]
    pub dynamics365_cds_activities: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dynamics365DataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(rename = "dataTypes")]
    pub data_types: Dynamics365DataConnectorDataTypes,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeAtpDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfficeAtpDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeAtpDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(flatten)]
    pub data_connector_with_alerts_properties: DataConnectorWithAlertsProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MdatpDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MdatpDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MdatpDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(flatten)]
    pub data_connector_with_alerts_properties: DataConnectorWithAlertsProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfficeDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeDataConnectorDataTypes {
    pub exchange: serde_json::Value,
    #[serde(rename = "sharePoint")]
    pub share_point: serde_json::Value,
    pub teams: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(rename = "dataTypes")]
    pub data_types: OfficeDataConnectorDataTypes,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TiDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiDataConnectorDataTypes {
    pub indicators: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(rename = "tipLookbackPeriod", default, skip_serializing_if = "Option::is_none")]
    pub tip_lookback_period: Option<String>,
    #[serde(rename = "dataTypes")]
    pub data_types: TiDataConnectorDataTypes,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiTaxiiDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TiTaxiiDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiTaxiiDataConnectorDataTypes {
    #[serde(rename = "taxiiClient")]
    pub taxii_client: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiTaxiiDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_tenant_id: DataConnectorTenantId,
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "taxiiServer", default, skip_serializing_if = "Option::is_none")]
    pub taxii_server: Option<String>,
    #[serde(rename = "collectionId", default, skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "taxiiLookbackPeriod", default, skip_serializing_if = "Option::is_none")]
    pub taxii_lookback_period: Option<String>,
    #[serde(rename = "pollingFrequency")]
    pub polling_frequency: ti_taxii_data_connector_properties::PollingFrequency,
    #[serde(rename = "dataTypes")]
    pub data_types: TiTaxiiDataConnectorDataTypes,
}
pub mod ti_taxii_data_connector_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PollingFrequency {
        OnceAMinute,
        OnceAnHour,
        OnceADay,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodelessUiDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CodelessParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodelessParameters {
    #[serde(rename = "connectorUiConfig", default, skip_serializing_if = "Option::is_none")]
    pub connector_ui_config: Option<CodelessUiConnectorConfigProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodelessUiConnectorConfigProperties {
    pub title: String,
    pub publisher: String,
    #[serde(rename = "descriptionMarkdown")]
    pub description_markdown: String,
    #[serde(rename = "customImage", default, skip_serializing_if = "Option::is_none")]
    pub custom_image: Option<String>,
    #[serde(rename = "graphQueriesTableName")]
    pub graph_queries_table_name: String,
    #[serde(rename = "graphQueries")]
    pub graph_queries: Vec<serde_json::Value>,
    #[serde(rename = "sampleQueries")]
    pub sample_queries: Vec<serde_json::Value>,
    #[serde(rename = "dataTypes")]
    pub data_types: Vec<serde_json::Value>,
    #[serde(rename = "connectivityCriteria")]
    pub connectivity_criteria: Vec<serde_json::Value>,
    pub availability: Availability,
    pub permissions: Permissions,
    #[serde(rename = "instructionSteps")]
    pub instruction_steps: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LastDataReceivedDataType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "lastDataReceivedQuery", default, skip_serializing_if = "Option::is_none")]
    pub last_data_received_query: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permissions {
    #[serde(rename = "resourceProvider", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_provider: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub customs: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Customs {
    #[serde(flatten)]
    pub customs_permission: CustomsPermission,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomsPermission {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProvider {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<resource_provider::Provider>,
    #[serde(rename = "permissionsDisplayText", default, skip_serializing_if = "Option::is_none")]
    pub permissions_display_text: Option<String>,
    #[serde(rename = "providerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub provider_display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<resource_provider::Scope>,
    #[serde(rename = "requiredPermissions", default, skip_serializing_if = "Option::is_none")]
    pub required_permissions: Option<RequiredPermissions>,
}
pub mod resource_provider {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Provider {
        #[serde(rename = "Microsoft.OperationalInsights/solutions")]
        MicrosoftOperationalInsightsSolutions,
        #[serde(rename = "Microsoft.OperationalInsights/workspaces")]
        MicrosoftOperationalInsightsWorkspaces,
        #[serde(rename = "Microsoft.OperationalInsights/workspaces/datasources")]
        MicrosoftOperationalInsightsWorkspacesDatasources,
        #[serde(rename = "microsoft.aadiam/diagnosticSettings")]
        MicrosoftAadiamDiagnosticSettings,
        #[serde(rename = "Microsoft.OperationalInsights/workspaces/sharedKeys")]
        MicrosoftOperationalInsightsWorkspacesSharedKeys,
        #[serde(rename = "Microsoft.Authorization/policyAssignments")]
        MicrosoftAuthorizationPolicyAssignments,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scope {
        ResourceGroup,
        Subscription,
        Workspace,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstructionSteps {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instructions: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectorInstructionModelBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: connector_instruction_model_base::Type,
}
pub mod connector_instruction_model_base {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        CopyableLabel,
        InstructionStepsGroup,
        InfoMessage,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequiredPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Availability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<availability::Status>,
    #[serde(rename = "isPreview", default, skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
}
pub mod availability {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {}
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectivityCriteria {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<connectivity_criteria::Type>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
pub mod connectivity_criteria {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        IsConnectedQuery,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SampleQueries {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphQueries {
    #[serde(rename = "metricName", default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legend: Option<String>,
    #[serde(rename = "baseQuery", default, skip_serializing_if = "Option::is_none")]
    pub base_query: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistList {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    pub value: Vec<Watchlist>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Watchlist {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WatchlistProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistProperties {
    #[serde(rename = "watchlistId", default, skip_serializing_if = "Option::is_none")]
    pub watchlist_id: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub provider: String,
    pub source: watchlist_properties::Source,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserInfo>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<UserInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "watchlistType", default, skip_serializing_if = "Option::is_none")]
    pub watchlist_type: Option<String>,
    #[serde(rename = "watchlistAlias", default, skip_serializing_if = "Option::is_none")]
    pub watchlist_alias: Option<String>,
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Label>,
    #[serde(rename = "defaultDuration", default, skip_serializing_if = "Option::is_none")]
    pub default_duration: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "numberOfLinesToSkip", default, skip_serializing_if = "Option::is_none")]
    pub number_of_lines_to_skip: Option<i32>,
    #[serde(rename = "rawContent", default, skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    #[serde(rename = "itemsSearchKey")]
    pub items_search_key: String,
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "uploadStatus", default, skip_serializing_if = "Option::is_none")]
    pub upload_status: Option<String>,
    #[serde(rename = "watchlistItemsCount", default, skip_serializing_if = "Option::is_none")]
    pub watchlist_items_count: Option<i32>,
}
pub mod watchlist_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Source {
        #[serde(rename = "Local file")]
        LocalFile,
        #[serde(rename = "Remote storage")]
        RemoteStorage,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistItemList {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    pub value: Vec<WatchlistItem>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistItem {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WatchlistItemProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistItemProperties {
    #[serde(rename = "watchlistItemType", default, skip_serializing_if = "Option::is_none")]
    pub watchlist_item_type: Option<String>,
    #[serde(rename = "watchlistItemId", default, skip_serializing_if = "Option::is_none")]
    pub watchlist_item_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserInfo>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<UserInfo>,
    #[serde(rename = "itemsKeyValue")]
    pub items_key_value: serde_json::Value,
    #[serde(rename = "entityMapping", default, skip_serializing_if = "Option::is_none")]
    pub entity_mapping: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(skip_serializing)]
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Label {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataList {
    pub value: Vec<MetadataModel>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataModel {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetadataProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataPatch {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetadataPropertiesPatch>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataContentId {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataParentId {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataVersion {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MetadataKind {
    #[serde(rename = "dataConnector")]
    DataConnector,
    #[serde(rename = "dataType")]
    DataType,
    #[serde(rename = "workbook")]
    Workbook,
    #[serde(rename = "workbookTemplate")]
    WorkbookTemplate,
    #[serde(rename = "playbook")]
    Playbook,
    #[serde(rename = "playbookTemplate")]
    PlaybookTemplate,
    #[serde(rename = "analyticRuleTemplate")]
    AnalyticRuleTemplate,
    #[serde(rename = "analyticRule")]
    AnalyticRule,
    #[serde(rename = "huntingQuery")]
    HuntingQuery,
    #[serde(rename = "investigationQuery")]
    InvestigationQuery,
    #[serde(rename = "parser")]
    Parser,
    #[serde(rename = "watchlist")]
    Watchlist,
    #[serde(rename = "watchlistTemplate")]
    WatchlistTemplate,
    #[serde(rename = "solution")]
    Solution,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataSource {
    pub kind: metadata_source::Kind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}
pub mod metadata_source {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "localWorkspace")]
        LocalWorkspace,
        #[serde(rename = "community")]
        Community,
        #[serde(rename = "solution")]
        Solution,
        #[serde(rename = "sourceRepository")]
        SourceRepository,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataAuthor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataSupport {
    pub tier: metadata_support::Tier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
pub mod metadata_support {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        #[serde(rename = "microsoft")]
        Microsoft,
        #[serde(rename = "developer")]
        Developer,
        #[serde(rename = "community")]
        Community,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataDependencies {
    #[serde(rename = "contentId", default, skip_serializing_if = "Option::is_none")]
    pub content_id: Option<MetadataContentId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MetadataKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<MetadataVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<metadata_dependencies::Operator>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub criteria: Vec<MetadataDependencies>,
}
pub mod metadata_dependencies {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        #[serde(rename = "AND")]
        And,
        #[serde(rename = "OR")]
        Or,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataProperties {
    #[serde(rename = "contentId")]
    pub content_id: MetadataContentId,
    #[serde(rename = "parentId")]
    pub parent_id: MetadataParentId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<MetadataVersion>,
    pub kind: MetadataKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<MetadataSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<MetadataAuthor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support: Option<MetadataSupport>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<MetadataDependencies>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataPropertiesPatch {
    #[serde(rename = "contentId", default, skip_serializing_if = "Option::is_none")]
    pub content_id: Option<MetadataContentId>,
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<MetadataParentId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<MetadataVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MetadataKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<MetadataSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<MetadataAuthor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support: Option<MetadataSupport>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<MetadataDependencies>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceWithEtag {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
