#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(rename = "clusterId", skip_serializing)]
    pub cluster_id: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<cluster_properties::ProvisioningState>,
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
}
pub mod cluster_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Succeeded,
        Failed,
        Canceled,
        Deleting,
        ProvisioningAccount,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterPatchProperties {
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterPatchProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultProperties {
    #[serde(rename = "keyVaultUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "keyVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<sku::Name>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        CapacityReservation,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: identity::Type,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        None,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedServiceProperties {
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "writeAccessResourceId", default, skip_serializing_if = "Option::is_none")]
    pub write_access_resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedService {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: LinkedServiceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedServiceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LinkedService>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExport {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataExportProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataExport>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportProperties {
    #[serde(rename = "dataExportId", default, skip_serializing_if = "Option::is_none")]
    pub data_export_id: Option<String>,
    #[serde(rename = "tableNames")]
    pub table_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "lastModifiedDate", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Destination {
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<destination::Type>,
    #[serde(rename = "metaData", default, skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<DestinationMetaData>,
}
pub mod destination {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        StorageAccount,
        EventHub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DestinationMetaData {
    #[serde(rename = "eventHubName", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedStorageAccountsProperties {
    #[serde(rename = "dataSourceType", skip_serializing)]
    pub data_source_type: Option<linked_storage_accounts_properties::DataSourceType>,
    #[serde(rename = "storageAccountIds", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_account_ids: Vec<String>,
}
pub mod linked_storage_accounts_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataSourceType {
        CustomLogs,
        AzureWatson,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedStorageAccounts {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: LinkedStorageAccountsProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedStorageAccountsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LinkedStorageAccounts>,
}