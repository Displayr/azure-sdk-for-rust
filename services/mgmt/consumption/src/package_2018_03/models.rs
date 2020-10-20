#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeterDetails {
    #[serde(rename = "meterName", skip_serializing)]
    pub meter_name: Option<String>,
    #[serde(rename = "meterCategory", skip_serializing)]
    pub meter_category: Option<String>,
    #[serde(rename = "meterSubCategory", skip_serializing)]
    pub meter_sub_category: Option<String>,
    #[serde(skip_serializing)]
    pub unit: Option<String>,
    #[serde(rename = "meterLocation", skip_serializing)]
    pub meter_location: Option<String>,
    #[serde(rename = "totalIncludedQuantity", skip_serializing)]
    pub total_included_quantity: Option<f64>,
    #[serde(rename = "pretaxStandardRate", skip_serializing)]
    pub pretax_standard_rate: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageDetail {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UsageDetailProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageDetailsListResult {
    #[serde(skip_serializing)]
    pub value: Vec<UsageDetail>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageDetailProperties {
    #[serde(rename = "billingPeriodId", skip_serializing)]
    pub billing_period_id: Option<String>,
    #[serde(rename = "invoiceId", skip_serializing)]
    pub invoice_id: Option<String>,
    #[serde(rename = "usageStart", skip_serializing)]
    pub usage_start: Option<String>,
    #[serde(rename = "usageEnd", skip_serializing)]
    pub usage_end: Option<String>,
    #[serde(rename = "instanceName", skip_serializing)]
    pub instance_name: Option<String>,
    #[serde(rename = "instanceId", skip_serializing)]
    pub instance_id: Option<String>,
    #[serde(rename = "instanceLocation", skip_serializing)]
    pub instance_location: Option<String>,
    #[serde(skip_serializing)]
    pub currency: Option<String>,
    #[serde(rename = "usageQuantity", skip_serializing)]
    pub usage_quantity: Option<f64>,
    #[serde(rename = "billableQuantity", skip_serializing)]
    pub billable_quantity: Option<f64>,
    #[serde(rename = "pretaxCost", skip_serializing)]
    pub pretax_cost: Option<f64>,
    #[serde(rename = "isEstimated", skip_serializing)]
    pub is_estimated: Option<bool>,
    #[serde(rename = "meterId", skip_serializing)]
    pub meter_id: Option<String>,
    #[serde(rename = "meterDetails", skip_serializing_if = "Option::is_none")]
    pub meter_details: Option<MeterDetails>,
    #[serde(rename = "subscriptionGuid", skip_serializing)]
    pub subscription_guid: Option<String>,
    #[serde(rename = "subscriptionName", skip_serializing)]
    pub subscription_name: Option<String>,
    #[serde(rename = "accountName", skip_serializing)]
    pub account_name: Option<String>,
    #[serde(rename = "departmentName", skip_serializing)]
    pub department_name: Option<String>,
    #[serde(skip_serializing)]
    pub product: Option<String>,
    #[serde(rename = "consumedService", skip_serializing)]
    pub consumed_service: Option<String>,
    #[serde(rename = "costCenter", skip_serializing)]
    pub cost_center: Option<String>,
    #[serde(rename = "additionalProperties", skip_serializing)]
    pub additional_properties: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Marketplace {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MarketplaceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplacesListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Marketplace>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceProperties {
    #[serde(rename = "billingPeriodId", skip_serializing)]
    pub billing_period_id: Option<String>,
    #[serde(rename = "usageStart", skip_serializing)]
    pub usage_start: Option<String>,
    #[serde(rename = "usageEnd", skip_serializing)]
    pub usage_end: Option<String>,
    #[serde(rename = "resourceRate", skip_serializing)]
    pub resource_rate: Option<f64>,
    #[serde(rename = "offerName", skip_serializing)]
    pub offer_name: Option<String>,
    #[serde(rename = "resourceGroup", skip_serializing)]
    pub resource_group: Option<String>,
    #[serde(rename = "orderNumber", skip_serializing)]
    pub order_number: Option<String>,
    #[serde(rename = "instanceName", skip_serializing)]
    pub instance_name: Option<String>,
    #[serde(rename = "instanceId", skip_serializing)]
    pub instance_id: Option<String>,
    #[serde(skip_serializing)]
    pub currency: Option<String>,
    #[serde(rename = "consumedQuantity", skip_serializing)]
    pub consumed_quantity: Option<f64>,
    #[serde(rename = "unitOfMeasure", skip_serializing)]
    pub unit_of_measure: Option<String>,
    #[serde(rename = "pretaxCost", skip_serializing)]
    pub pretax_cost: Option<f64>,
    #[serde(rename = "isEstimated", skip_serializing)]
    pub is_estimated: Option<bool>,
    #[serde(rename = "meterId", skip_serializing)]
    pub meter_id: Option<String>,
    #[serde(rename = "subscriptionGuid", skip_serializing)]
    pub subscription_guid: Option<String>,
    #[serde(rename = "subscriptionName", skip_serializing)]
    pub subscription_name: Option<String>,
    #[serde(rename = "accountName", skip_serializing)]
    pub account_name: Option<String>,
    #[serde(rename = "departmentName", skip_serializing)]
    pub department_name: Option<String>,
    #[serde(rename = "consumedService", skip_serializing)]
    pub consumed_service: Option<String>,
    #[serde(rename = "costCenter", skip_serializing)]
    pub cost_center: Option<String>,
    #[serde(rename = "additionalProperties", skip_serializing)]
    pub additional_properties: Option<String>,
    #[serde(rename = "publisherName", skip_serializing)]
    pub publisher_name: Option<String>,
    #[serde(rename = "planName", skip_serializing)]
    pub plan_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Balance {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BalanceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BalanceProperties {
    #[serde(skip_serializing)]
    pub currency: Option<String>,
    #[serde(rename = "beginningBalance", skip_serializing)]
    pub beginning_balance: Option<f64>,
    #[serde(rename = "endingBalance", skip_serializing)]
    pub ending_balance: Option<f64>,
    #[serde(rename = "newPurchases", skip_serializing)]
    pub new_purchases: Option<f64>,
    #[serde(skip_serializing)]
    pub adjustments: Option<f64>,
    #[serde(skip_serializing)]
    pub utilized: Option<f64>,
    #[serde(rename = "serviceOverage", skip_serializing)]
    pub service_overage: Option<f64>,
    #[serde(rename = "chargesBilledSeparately", skip_serializing)]
    pub charges_billed_separately: Option<f64>,
    #[serde(rename = "totalOverage", skip_serializing)]
    pub total_overage: Option<f64>,
    #[serde(rename = "totalUsage", skip_serializing)]
    pub total_usage: Option<f64>,
    #[serde(rename = "azureMarketplaceServiceCharges", skip_serializing)]
    pub azure_marketplace_service_charges: Option<f64>,
    #[serde(rename = "billingFrequency", skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<balance_properties::BillingFrequency>,
    #[serde(rename = "priceHidden", skip_serializing)]
    pub price_hidden: Option<bool>,
    #[serde(rename = "newPurchasesDetails", skip_serializing)]
    pub new_purchases_details: Vec<serde_json::Value>,
    #[serde(rename = "adjustmentDetails", skip_serializing)]
    pub adjustment_details: Vec<serde_json::Value>,
}
mod balance_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BillingFrequency {
        Month,
        Quarter,
        Year,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationSummaries {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationSummariesProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationSummariesListResult {
    #[serde(skip_serializing)]
    pub value: Vec<ReservationSummaries>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationSummariesProperties {
    #[serde(rename = "reservationOrderId", skip_serializing)]
    pub reservation_order_id: Option<String>,
    #[serde(rename = "reservationId", skip_serializing)]
    pub reservation_id: Option<String>,
    #[serde(rename = "skuName", skip_serializing)]
    pub sku_name: Option<String>,
    #[serde(rename = "reservedHours", skip_serializing)]
    pub reserved_hours: Option<f64>,
    #[serde(rename = "usageDate", skip_serializing)]
    pub usage_date: Option<String>,
    #[serde(rename = "usedHours", skip_serializing)]
    pub used_hours: Option<f64>,
    #[serde(rename = "minUtilizationPercentage", skip_serializing)]
    pub min_utilization_percentage: Option<f64>,
    #[serde(rename = "avgUtilizationPercentage", skip_serializing)]
    pub avg_utilization_percentage: Option<f64>,
    #[serde(rename = "maxUtilizationPercentage", skip_serializing)]
    pub max_utilization_percentage: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationDetails {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationDetailsProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationDetailsListResult {
    #[serde(skip_serializing)]
    pub value: Vec<ReservationDetails>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationRecommendationsProperties {
    #[serde(rename = "lookBackPeriod", skip_serializing)]
    pub look_back_period: Option<String>,
    #[serde(rename = "meterId", skip_serializing)]
    pub meter_id: Option<String>,
    #[serde(skip_serializing)]
    pub term: Option<String>,
    #[serde(rename = "costWithNoReservedInstances", skip_serializing)]
    pub cost_with_no_reserved_instances: Option<f64>,
    #[serde(rename = "recommendedQuantity", skip_serializing)]
    pub recommended_quantity: Option<f64>,
    #[serde(rename = "totalCostWithReservedInstances", skip_serializing)]
    pub total_cost_with_reserved_instances: Option<f64>,
    #[serde(rename = "netSavings", skip_serializing)]
    pub net_savings: Option<f64>,
    #[serde(rename = "firstUsageDate", skip_serializing)]
    pub first_usage_date: Option<String>,
    #[serde(skip_serializing)]
    pub scope: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationRecommendations {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub resource_attributes: ResourceAttributes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationRecommendationsProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationRecommendationsListResult {
    #[serde(skip_serializing)]
    pub value: Vec<ReservationRecommendations>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationDetailsProperties {
    #[serde(rename = "reservationOrderId", skip_serializing)]
    pub reservation_order_id: Option<String>,
    #[serde(rename = "reservationId", skip_serializing)]
    pub reservation_id: Option<String>,
    #[serde(rename = "skuName", skip_serializing)]
    pub sku_name: Option<String>,
    #[serde(rename = "reservedHours", skip_serializing)]
    pub reserved_hours: Option<f64>,
    #[serde(rename = "usageDate", skip_serializing)]
    pub usage_date: Option<String>,
    #[serde(rename = "usedHours", skip_serializing)]
    pub used_hours: Option<f64>,
    #[serde(rename = "instanceId", skip_serializing)]
    pub instance_id: Option<String>,
    #[serde(rename = "totalReservedQuantity", skip_serializing)]
    pub total_reserved_quantity: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tags {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TagProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagProperties {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetsListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Budget>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Budget {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BudgetProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetProperties {
    pub category: budget_properties::Category,
    pub amount: f64,
    #[serde(rename = "timeGrain")]
    pub time_grain: budget_properties::TimeGrain,
    #[serde(rename = "timePeriod")]
    pub time_period: BudgetTimePeriod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,
    #[serde(rename = "currentSpend", skip_serializing_if = "Option::is_none")]
    pub current_spend: Option<CurrentSpend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<serde_json::Value>,
}
mod budget_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Category {
        Cost,
        Usage,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TimeGrain {
        Monthly,
        Quarterly,
        Annually,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetTimePeriod {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Filters {
    #[serde(rename = "resourceGroups", skip_serializing_if = "Vec::is_empty")]
    pub resource_groups: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub meters: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentSpend {
    #[serde(skip_serializing)]
    pub amount: Option<f64>,
    #[serde(skip_serializing)]
    pub unit: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub enabled: bool,
    pub operator: notification::Operator,
    pub threshold: f64,
    #[serde(rename = "contactEmails")]
    pub contact_emails: Vec<String>,
    #[serde(rename = "contactRoles", skip_serializing_if = "Vec::is_empty")]
    pub contact_roles: Vec<String>,
    #[serde(rename = "contactGroups", skip_serializing_if = "Vec::is_empty")]
    pub contact_groups: Vec<String>,
}
mod notification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        EqualTo,
        GreaterThan,
        GreaterThanOrEqualTo,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostTags {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CostTagProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostTagProperties {
    #[serde(rename = "costTags", skip_serializing_if = "Vec::is_empty")]
    pub cost_tags: Vec<CostTag>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostTag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing)]
        pub provider: Option<String>,
        #[serde(skip_serializing)]
        pub resource: Option<String>,
        #[serde(skip_serializing)]
        pub operation: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
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
    #[serde(skip_serializing)]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceAttributes {
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub sku: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(rename = "eTag", skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceSheetResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PriceSheetModel>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceSheetModel {
    #[serde(skip_serializing)]
    pub pricesheets: Vec<PriceSheetProperties>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceSheetProperties {
    #[serde(rename = "billingPeriodId", skip_serializing)]
    pub billing_period_id: Option<String>,
    #[serde(rename = "meterId", skip_serializing)]
    pub meter_id: Option<String>,
    #[serde(rename = "meterDetails", skip_serializing_if = "Option::is_none")]
    pub meter_details: Option<MeterDetails>,
    #[serde(rename = "unitOfMeasure", skip_serializing)]
    pub unit_of_measure: Option<String>,
    #[serde(rename = "includedQuantity", skip_serializing)]
    pub included_quantity: Option<f64>,
    #[serde(rename = "partNumber", skip_serializing)]
    pub part_number: Option<String>,
    #[serde(rename = "unitPrice", skip_serializing)]
    pub unit_price: Option<f64>,
    #[serde(rename = "currencyCode", skip_serializing)]
    pub currency_code: Option<String>,
}
