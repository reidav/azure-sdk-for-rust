#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyOnlyResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSettingsCategory {
    #[serde(rename = "categoryType", default, skip_serializing_if = "Option::is_none")]
    pub category_type: Option<diagnostic_settings_category::CategoryType>,
}
pub mod diagnostic_settings_category {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CategoryType {
        Metrics,
        Logs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSettingsCategoryResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticSettingsCategory>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSettingsCategoryResourceCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DiagnosticSettingsCategoryResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub enabled: bool,
    pub days: i32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSettings {
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub enabled: bool,
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub enabled: bool,
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSettings {
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[serde(rename = "serviceBusRuleId", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_rule_id: Option<String>,
    #[serde(rename = "eventHubAuthorizationRuleId", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_authorization_rule_id: Option<String>,
    #[serde(rename = "eventHubName", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<MetricSettings>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub logs: Vec<LogSettings>,
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "logAnalyticsDestinationType", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_destination_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSettingsResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSettingsResourceCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DiagnosticSettingsResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizableString {
    pub value: String,
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAvailability {
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Unit {
    Count,
    Bytes,
    Seconds,
    CountPerSecond,
    BytesPerSecond,
    Percent,
    MilliSeconds,
    ByteSeconds,
    Unspecified,
    Cores,
    MilliCores,
    NanoCores,
    BitsPerSecond,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinition {
    #[serde(rename = "isDimensionRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_dimension_required: Option<bool>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    #[serde(rename = "primaryAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub primary_aggregation_type: Option<metric_definition::PrimaryAggregationType>,
    #[serde(rename = "metricAvailabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_availabilities: Vec<MetricAvailability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<LocalizableString>,
}
pub mod metric_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrimaryAggregationType {
        None,
        Average,
        Count,
        Minimum,
        Maximum,
        Total,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinitionCollection {
    pub value: Vec<MetricDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricValue {
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    pub timespan: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    pub value: Vec<Metric>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub name: LocalizableString,
    #[serde(rename = "displayDescription")]
    pub display_description: String,
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    pub unit: Unit,
    pub timeseries: Vec<TimeSeriesElement>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesElement {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadatavalues: Vec<MetadataValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<MetricValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionProxyOnlyResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionLogSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub enabled: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionDiagnosticSettings {
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[serde(rename = "serviceBusRuleId", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_rule_id: Option<String>,
    #[serde(rename = "eventHubAuthorizationRuleId", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_authorization_rule_id: Option<String>,
    #[serde(rename = "eventHubName", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub logs: Vec<SubscriptionLogSettings>,
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionDiagnosticSettingsResource {
    #[serde(flatten)]
    pub subscription_proxy_only_resource: SubscriptionProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionDiagnosticSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionDiagnosticSettingsResourceCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubscriptionDiagnosticSettingsResource>,
}