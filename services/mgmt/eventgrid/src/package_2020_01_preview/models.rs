#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<domain_properties::ProvisioningState>,
    #[serde(skip_serializing)]
    pub endpoint: Option<String>,
    #[serde(rename = "inputSchema", skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<domain_properties::InputSchema>,
    #[serde(rename = "inputSchemaMapping", skip_serializing_if = "Option::is_none")]
    pub input_schema_mapping: Option<InputSchemaMapping>,
    #[serde(rename = "metricResourceId", skip_serializing)]
    pub metric_resource_id: Option<String>,
}
mod domain_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum InputSchema {
        EventGridSchema,
        CustomEventSchema,
        #[serde(rename = "CloudEventSchemaV1_0")]
        CloudEventSchemaV10,
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
pub struct JsonInputSchemaMappingProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<JsonField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<JsonField>,
    #[serde(rename = "eventTime", skip_serializing_if = "Option::is_none")]
    pub event_time: Option<JsonField>,
    #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<JsonFieldWithDefault>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<JsonFieldWithDefault>,
    #[serde(rename = "dataVersion", skip_serializing_if = "Option::is_none")]
    pub data_version: Option<JsonFieldWithDefault>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonField {
    #[serde(rename = "sourceField", skip_serializing_if = "Option::is_none")]
    pub source_field: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonFieldWithDefault {
    #[serde(rename = "sourceField", skip_serializing_if = "Option::is_none")]
    pub source_field: Option<String>,
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonInputSchemaMapping {
    #[serde(flatten)]
    pub input_schema_mapping: InputSchemaMapping,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<JsonInputSchemaMappingProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputSchemaMapping {
    #[serde(rename = "inputSchemaMappingType", skip_serializing_if = "Option::is_none")]
    pub input_schema_mapping_type: Option<input_schema_mapping::InputSchemaMappingType>,
}
mod input_schema_mapping {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum InputSchemaMappingType {
        Json,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Domain {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DomainProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainsListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Domain>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainSharedAccessKeys {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainRegenerateKeyRequest {
    #[serde(rename = "keyName")]
    pub key_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainTopicProperties {
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<domain_topic_properties::ProvisioningState>,
}
mod domain_topic_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainTopic {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DomainTopicProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainTopicsListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DomainTopic>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSubscriptionProperties {
    #[serde(skip_serializing)]
    pub topic: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<event_subscription_properties::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<EventSubscriptionDestination>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<EventSubscriptionFilter>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(rename = "expirationTimeUtc", skip_serializing_if = "Option::is_none")]
    pub expiration_time_utc: Option<String>,
    #[serde(rename = "eventDeliverySchema", skip_serializing_if = "Option::is_none")]
    pub event_delivery_schema: Option<event_subscription_properties::EventDeliverySchema>,
    #[serde(rename = "retryPolicy", skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    #[serde(rename = "deadLetterDestination", skip_serializing_if = "Option::is_none")]
    pub dead_letter_destination: Option<DeadLetterDestination>,
}
mod event_subscription_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
        AwaitingManualAction,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EventDeliverySchema {
        EventGridSchema,
        CustomInputSchema,
        #[serde(rename = "CloudEventSchemaV1_0")]
        CloudEventSchemaV10,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSubscriptionFilter {
    #[serde(rename = "subjectBeginsWith", skip_serializing_if = "Option::is_none")]
    pub subject_begins_with: Option<String>,
    #[serde(rename = "subjectEndsWith", skip_serializing_if = "Option::is_none")]
    pub subject_ends_with: Option<String>,
    #[serde(rename = "includedEventTypes", skip_serializing_if = "Vec::is_empty")]
    pub included_event_types: Vec<String>,
    #[serde(rename = "isSubjectCaseSensitive", skip_serializing_if = "Option::is_none")]
    pub is_subject_case_sensitive: Option<bool>,
    #[serde(rename = "advancedFilters", skip_serializing_if = "Vec::is_empty")]
    pub advanced_filters: Vec<AdvancedFilter>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetryPolicy {
    #[serde(rename = "maxDeliveryAttempts", skip_serializing_if = "Option::is_none")]
    pub max_delivery_attempts: Option<i32>,
    #[serde(rename = "eventTimeToLiveInMinutes", skip_serializing_if = "Option::is_none")]
    pub event_time_to_live_in_minutes: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebHookEventSubscriptionDestinationProperties {
    #[serde(rename = "endpointUrl", skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[serde(rename = "endpointBaseUrl", skip_serializing)]
    pub endpoint_base_url: Option<String>,
    #[serde(rename = "maxEventsPerBatch", skip_serializing_if = "Option::is_none")]
    pub max_events_per_batch: Option<i32>,
    #[serde(rename = "preferredBatchSizeInKilobytes", skip_serializing_if = "Option::is_none")]
    pub preferred_batch_size_in_kilobytes: Option<i32>,
    #[serde(rename = "azureActiveDirectoryTenantId", skip_serializing_if = "Option::is_none")]
    pub azure_active_directory_tenant_id: Option<String>,
    #[serde(rename = "azureActiveDirectoryApplicationIdOrUri", skip_serializing_if = "Option::is_none")]
    pub azure_active_directory_application_id_or_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageBlobDeadLetterDestinationProperties {
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "blobContainerName", skip_serializing_if = "Option::is_none")]
    pub blob_container_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberInAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageBlobDeadLetterDestination {
    #[serde(flatten)]
    pub dead_letter_destination: DeadLetterDestination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageBlobDeadLetterDestinationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberNotInAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberLessThanAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberGreaterThanAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberLessThanOrEqualsAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberGreaterThanOrEqualsAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoolEqualsAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringInAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringNotInAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringBeginsWithAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringEndsWithAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringContainsAdvancedFilter {
    #[serde(flatten)]
    pub advanced_filter: AdvancedFilter,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdvancedFilter {
    #[serde(rename = "operatorType")]
    pub operator_type: advanced_filter::OperatorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
mod advanced_filter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperatorType {
        NumberIn,
        NumberNotIn,
        NumberLessThan,
        NumberGreaterThan,
        NumberLessThanOrEquals,
        NumberGreaterThanOrEquals,
        BoolEquals,
        StringIn,
        StringNotIn,
        StringBeginsWith,
        StringEndsWith,
        StringContains,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebHookEventSubscriptionDestination {
    #[serde(flatten)]
    pub event_subscription_destination: EventSubscriptionDestination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebHookEventSubscriptionDestinationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeadLetterDestination {
    #[serde(rename = "endpointType")]
    pub endpoint_type: dead_letter_destination::EndpointType,
}
mod dead_letter_destination {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EndpointType {
        StorageBlob,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSubscriptionDestinationProperties {
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSubscriptionDestination {
    #[serde(flatten)]
    pub event_subscription_destination: EventSubscriptionDestination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubEventSubscriptionDestinationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageQueueEventSubscriptionDestinationProperties {
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "queueName", skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageQueueEventSubscriptionDestination {
    #[serde(flatten)]
    pub event_subscription_destination: EventSubscriptionDestination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageQueueEventSubscriptionDestinationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridConnectionEventSubscriptionDestinationProperties {
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridConnectionEventSubscriptionDestination {
    #[serde(flatten)]
    pub event_subscription_destination: EventSubscriptionDestination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HybridConnectionEventSubscriptionDestinationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusQueueEventSubscriptionDestinationProperties {
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusQueueEventSubscriptionDestination {
    #[serde(flatten)]
    pub event_subscription_destination: EventSubscriptionDestination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceBusQueueEventSubscriptionDestinationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusTopicEventSubscriptionDestinationProperties {
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusTopicEventSubscriptionDestination {
    #[serde(flatten)]
    pub event_subscription_destination: EventSubscriptionDestination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceBusTopicEventSubscriptionDestinationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFunctionEventSubscriptionDestinationProperties {
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "maxEventsPerBatch", skip_serializing_if = "Option::is_none")]
    pub max_events_per_batch: Option<i32>,
    #[serde(rename = "preferredBatchSizeInKilobytes", skip_serializing_if = "Option::is_none")]
    pub preferred_batch_size_in_kilobytes: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFunctionEventSubscriptionDestination {
    #[serde(flatten)]
    pub event_subscription_destination: EventSubscriptionDestination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureFunctionEventSubscriptionDestinationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSubscriptionDestination {
    #[serde(rename = "endpointType")]
    pub endpoint_type: event_subscription_destination::EndpointType,
}
mod event_subscription_destination {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EndpointType {
        WebHook,
        EventHub,
        StorageQueue,
        HybridConnection,
        ServiceBusQueue,
        ServiceBusTopic,
        AzureFunction,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSubscription {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventSubscriptionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSubscriptionUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<EventSubscriptionDestination>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<EventSubscriptionFilter>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(rename = "expirationTimeUtc", skip_serializing_if = "Option::is_none")]
    pub expiration_time_utc: Option<String>,
    #[serde(rename = "eventDeliverySchema", skip_serializing_if = "Option::is_none")]
    pub event_delivery_schema: Option<event_subscription_update_parameters::EventDeliverySchema>,
    #[serde(rename = "retryPolicy", skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    #[serde(rename = "deadLetterDestination", skip_serializing_if = "Option::is_none")]
    pub dead_letter_destination: Option<DeadLetterDestination>,
}
mod event_subscription_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EventDeliverySchema {
        EventGridSchema,
        CustomInputSchema,
        #[serde(rename = "CloudEventSchemaV1_0")]
        CloudEventSchemaV10,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSubscriptionFullUrl {
    #[serde(rename = "endpointUrl", skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSubscriptionsListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EventSubscription>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<topic_properties::ProvisioningState>,
    #[serde(skip_serializing)]
    pub endpoint: Option<String>,
    #[serde(rename = "inputSchema", skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<topic_properties::InputSchema>,
    #[serde(rename = "inputSchemaMapping", skip_serializing_if = "Option::is_none")]
    pub input_schema_mapping: Option<InputSchemaMapping>,
    #[serde(rename = "metricResourceId", skip_serializing)]
    pub metric_resource_id: Option<String>,
}
mod topic_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum InputSchema {
        EventGridSchema,
        CustomEventSchema,
        #[serde(rename = "CloudEventSchemaV1_0")]
        CloudEventSchemaV10,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TopicProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicsListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Topic>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicSharedAccessKeys {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicRegenerateKeyRequest {
    #[serde(rename = "keyName")]
    pub key_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventTypesListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EventType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventTypeProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "schemaUrl", skip_serializing_if = "Option::is_none")]
    pub schema_url: Option<String>,
    #[serde(rename = "isInDefaultSet", skip_serializing_if = "Option::is_none")]
    pub is_in_default_set: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventType {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventTypeProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicTypesListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TopicTypeInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicTypeProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "resourceRegionType", skip_serializing_if = "Option::is_none")]
    pub resource_region_type: Option<topic_type_properties::ResourceRegionType>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<topic_type_properties::ProvisioningState>,
    #[serde(rename = "supportedLocations", skip_serializing_if = "Vec::is_empty")]
    pub supported_locations: Vec<String>,
}
mod topic_type_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceRegionType {
        RegionalResource,
        GlobalResource,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicTypeInfo {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TopicTypeProperties>,
}
