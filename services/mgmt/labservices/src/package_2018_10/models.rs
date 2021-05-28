#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddUsersPayload {
    #[serde(rename = "emailAddresses")]
    pub email_addresses: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateLabProperties {
    #[serde(rename = "environmentSettingCreationParameters", default, skip_serializing_if = "Option::is_none")]
    pub environment_setting_creation_parameters: Option<EnvironmentSettingCreationParameters>,
    #[serde(rename = "labCreationParameters")]
    pub lab_creation_parameters: LabCreationParameters,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Environment {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentDetails {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "virtualMachineDetails", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_details: Option<VirtualMachineDetails>,
    #[serde(rename = "latestOperationResult", default, skip_serializing_if = "Option::is_none")]
    pub latest_operation_result: Option<LatestOperationResult>,
    #[serde(rename = "environmentState", skip_serializing)]
    pub environment_state: Option<String>,
    #[serde(rename = "totalUsage", skip_serializing)]
    pub total_usage: Option<String>,
    #[serde(rename = "passwordLastReset", skip_serializing)]
    pub password_last_reset: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentFragment {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentPropertiesFragment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentOperationsPayload {
    #[serde(rename = "environmentId")]
    pub environment_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentProperties {
    #[serde(rename = "resourceSets", default, skip_serializing_if = "Option::is_none")]
    pub resource_sets: Option<ResourceSet>,
    #[serde(rename = "claimedByUserObjectId", skip_serializing)]
    pub claimed_by_user_object_id: Option<String>,
    #[serde(rename = "claimedByUserPrincipalId", skip_serializing)]
    pub claimed_by_user_principal_id: Option<String>,
    #[serde(rename = "claimedByUserName", skip_serializing)]
    pub claimed_by_user_name: Option<String>,
    #[serde(rename = "isClaimed", skip_serializing)]
    pub is_claimed: Option<bool>,
    #[serde(rename = "lastKnownPowerState", skip_serializing)]
    pub last_known_power_state: Option<String>,
    #[serde(rename = "networkInterface", default, skip_serializing_if = "Option::is_none")]
    pub network_interface: Option<NetworkInterface>,
    #[serde(rename = "totalUsage", skip_serializing)]
    pub total_usage: Option<String>,
    #[serde(rename = "passwordLastReset", skip_serializing)]
    pub password_last_reset: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
    #[serde(rename = "latestOperationResult", default, skip_serializing_if = "Option::is_none")]
    pub latest_operation_result: Option<LatestOperationResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentPropertiesFragment {
    #[serde(rename = "resourceSets", default, skip_serializing_if = "Option::is_none")]
    pub resource_sets: Option<ResourceSetFragment>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentSetting {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentSettingProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentSettingCreationParameters {
    #[serde(rename = "resourceSettingCreationParameters")]
    pub resource_setting_creation_parameters: ResourceSettingCreationParameters,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentSettingFragment {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentSettingPropertiesFragment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentSettingProperties {
    #[serde(rename = "publishingState", skip_serializing)]
    pub publishing_state: Option<environment_setting_properties::PublishingState>,
    #[serde(rename = "configurationState", default, skip_serializing_if = "Option::is_none")]
    pub configuration_state: Option<environment_setting_properties::ConfigurationState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "resourceSettings")]
    pub resource_settings: ResourceSettings,
    #[serde(rename = "lastChanged", skip_serializing)]
    pub last_changed: Option<String>,
    #[serde(rename = "lastPublished", skip_serializing)]
    pub last_published: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
    #[serde(rename = "latestOperationResult", default, skip_serializing_if = "Option::is_none")]
    pub latest_operation_result: Option<LatestOperationResult>,
}
pub mod environment_setting_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublishingState {
        Draft,
        Publishing,
        Published,
        PublishFailed,
        Scaling,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ConfigurationState {
        NotApplicable,
        Completed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentSettingPropertiesFragment {
    #[serde(rename = "configurationState", default, skip_serializing_if = "Option::is_none")]
    pub configuration_state: Option<environment_setting_properties_fragment::ConfigurationState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "resourceSettings", default, skip_serializing_if = "Option::is_none")]
    pub resource_settings: Option<ResourceSettingsFragment>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
pub mod environment_setting_properties_fragment {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ConfigurationState {
        NotApplicable,
        Completed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentSize {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<environment_size::Name>,
    #[serde(rename = "vmSizes", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_sizes: Vec<SizeInfo>,
    #[serde(rename = "maxPrice", skip_serializing)]
    pub max_price: Option<f64>,
    #[serde(rename = "minNumberOfCores", skip_serializing)]
    pub min_number_of_cores: Option<i32>,
    #[serde(rename = "minMemory", skip_serializing)]
    pub min_memory: Option<f64>,
}
pub mod environment_size {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Basic,
        Standard,
        Performance,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentSizeFragment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<environment_size_fragment::Name>,
    #[serde(rename = "vmSizes", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_sizes: Vec<SizeInfoFragment>,
}
pub mod environment_size_fragment {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Basic,
        Standard,
        Performance,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImage {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryImageProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageFragment {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryImagePropertiesFragment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageProperties {
    #[serde(skip_serializing)]
    pub author: Option<String>,
    #[serde(rename = "createdDate", skip_serializing)]
    pub created_date: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<GalleryImageReference>,
    #[serde(skip_serializing)]
    pub icon: Option<String>,
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "isOverride", default, skip_serializing_if = "Option::is_none")]
    pub is_override: Option<bool>,
    #[serde(rename = "planId", skip_serializing)]
    pub plan_id: Option<String>,
    #[serde(rename = "isPlanAuthorized", default, skip_serializing_if = "Option::is_none")]
    pub is_plan_authorized: Option<bool>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
    #[serde(rename = "latestOperationResult", default, skip_serializing_if = "Option::is_none")]
    pub latest_operation_result: Option<LatestOperationResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImagePropertiesFragment {
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "isOverride", default, skip_serializing_if = "Option::is_none")]
    pub is_override: Option<bool>,
    #[serde(rename = "isPlanAuthorized", default, skip_serializing_if = "Option::is_none")]
    pub is_plan_authorized: Option<bool>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageReferenceFragment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<EnvironmentDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetPersonalPreferencesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "favoriteLabResourceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub favorite_lab_resource_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetRegionalAvailabilityResponse {
    #[serde(rename = "regionalAvailability", default, skip_serializing_if = "Vec::is_empty")]
    pub regional_availability: Vec<RegionalAvailability>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lab {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabAccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabAccountFragment {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabAccountPropertiesFragment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabAccountProperties {
    #[serde(rename = "sizeConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub size_configuration: Option<SizeConfigurationProperties>,
    #[serde(rename = "enabledRegionSelection", default, skip_serializing_if = "Option::is_none")]
    pub enabled_region_selection: Option<bool>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
    #[serde(rename = "latestOperationResult", default, skip_serializing_if = "Option::is_none")]
    pub latest_operation_result: Option<LatestOperationResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabAccountPropertiesFragment {
    #[serde(rename = "enabledRegionSelection", default, skip_serializing_if = "Option::is_none")]
    pub enabled_region_selection: Option<bool>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabCreationParameters {
    #[serde(rename = "maxUsersInLab", default, skip_serializing_if = "Option::is_none")]
    pub max_users_in_lab: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "usageQuota", skip_serializing)]
    pub usage_quota: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabFragment {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabPropertiesFragment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabProperties {
    #[serde(rename = "maxUsersInLab", default, skip_serializing_if = "Option::is_none")]
    pub max_users_in_lab: Option<i32>,
    #[serde(rename = "userQuota", skip_serializing)]
    pub user_quota: Option<i32>,
    #[serde(rename = "invitationCode", skip_serializing)]
    pub invitation_code: Option<String>,
    #[serde(rename = "createdByObjectId", skip_serializing)]
    pub created_by_object_id: Option<String>,
    #[serde(rename = "usageQuota", default, skip_serializing_if = "Option::is_none")]
    pub usage_quota: Option<String>,
    #[serde(rename = "userAccessMode", default, skip_serializing_if = "Option::is_none")]
    pub user_access_mode: Option<lab_properties::UserAccessMode>,
    #[serde(rename = "createdByUserPrincipalName", skip_serializing)]
    pub created_by_user_principal_name: Option<String>,
    #[serde(rename = "createdDate", skip_serializing)]
    pub created_date: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
    #[serde(rename = "latestOperationResult", default, skip_serializing_if = "Option::is_none")]
    pub latest_operation_result: Option<LatestOperationResult>,
}
pub mod lab_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UserAccessMode {
        Restricted,
        Open,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabPropertiesFragment {
    #[serde(rename = "maxUsersInLab", default, skip_serializing_if = "Option::is_none")]
    pub max_users_in_lab: Option<i32>,
    #[serde(rename = "usageQuota", default, skip_serializing_if = "Option::is_none")]
    pub usage_quota: Option<String>,
    #[serde(rename = "userAccessMode", default, skip_serializing_if = "Option::is_none")]
    pub user_access_mode: Option<lab_properties_fragment::UserAccessMode>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
pub mod lab_properties_fragment {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UserAccessMode {
        Restricted,
        Open,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LatestOperationResult {
    #[serde(skip_serializing)]
    pub status: Option<String>,
    #[serde(rename = "errorCode", skip_serializing)]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage", skip_serializing)]
    pub error_message: Option<String>,
    #[serde(rename = "requestUri", skip_serializing)]
    pub request_uri: Option<String>,
    #[serde(rename = "httpMethod", skip_serializing)]
    pub http_method: Option<String>,
    #[serde(rename = "operationUrl", skip_serializing)]
    pub operation_url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LatestOperationResultFragment {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListEnvironmentsPayload {
    #[serde(rename = "labId", default, skip_serializing_if = "Option::is_none")]
    pub lab_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListEnvironmentsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environments: Vec<EnvironmentDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListLabsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labs: Vec<LabDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkInterface {
    #[serde(rename = "privateIpAddress", skip_serializing)]
    pub private_ip_address: Option<String>,
    #[serde(rename = "sshAuthority", skip_serializing)]
    pub ssh_authority: Option<String>,
    #[serde(rename = "rdpAuthority", skip_serializing)]
    pub rdp_authority: Option<String>,
    #[serde(skip_serializing)]
    pub username: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkInterfaceFragment {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationBatchStatusPayload {
    pub urls: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationBatchStatusResponse {
    #[serde(skip_serializing)]
    pub items: Vec<OperationBatchStatusResponseItem>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationBatchStatusResponseItem {
    #[serde(rename = "operationUrl", skip_serializing)]
    pub operation_url: Option<String>,
    #[serde(skip_serializing)]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationMetadataDisplay>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationMetadataDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusPayload {
    #[serde(rename = "operationUrl")]
    pub operation_url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusResponse {
    #[serde(skip_serializing)]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonalPreferencesOperationsPayload {
    #[serde(rename = "labAccountResourceId", default, skip_serializing_if = "Option::is_none")]
    pub lab_account_resource_id: Option<String>,
    #[serde(rename = "addRemove", default, skip_serializing_if = "Option::is_none")]
    pub add_remove: Option<personal_preferences_operations_payload::AddRemove>,
    #[serde(rename = "labResourceId", default, skip_serializing_if = "Option::is_none")]
    pub lab_resource_id: Option<String>,
}
pub mod personal_preferences_operations_payload {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AddRemove {
        Add,
        Remove,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderOperationResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationMetadata>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishPayload {
    #[serde(rename = "useExistingImage", default, skip_serializing_if = "Option::is_none")]
    pub use_existing_image: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceVm {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "vmStateDetails", default, skip_serializing_if = "Option::is_none")]
    pub vm_state_details: Option<VmStateDetails>,
    #[serde(rename = "vmResourceId", skip_serializing)]
    pub vm_resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceVmCreationParameters {
    #[serde(rename = "userName")]
    pub user_name: String,
    pub password: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceVmFragment {
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionalAvailability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "sizeAvailabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub size_availabilities: Vec<SizeAvailability>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterPayload {
    #[serde(rename = "registrationCode", default, skip_serializing_if = "Option::is_none")]
    pub registration_code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResetPasswordPayload {
    #[serde(rename = "environmentId")]
    pub environment_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
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
pub struct ResourceSet {
    #[serde(rename = "vmResourceId", default, skip_serializing_if = "Option::is_none")]
    pub vm_resource_id: Option<String>,
    #[serde(rename = "resourceSettingId", default, skip_serializing_if = "Option::is_none")]
    pub resource_setting_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSetFragment {
    #[serde(rename = "vmResourceId", default, skip_serializing_if = "Option::is_none")]
    pub vm_resource_id: Option<String>,
    #[serde(rename = "resourceSettingId", default, skip_serializing_if = "Option::is_none")]
    pub resource_setting_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSettingCreationParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "galleryImageResourceId")]
    pub gallery_image_resource_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<resource_setting_creation_parameters::Size>,
    #[serde(rename = "referenceVmCreationParameters")]
    pub reference_vm_creation_parameters: ReferenceVmCreationParameters,
}
pub mod resource_setting_creation_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Size {
        Basic,
        Standard,
        Performance,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSettings {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "galleryImageResourceId", default, skip_serializing_if = "Option::is_none")]
    pub gallery_image_resource_id: Option<String>,
    #[serde(rename = "imageName", skip_serializing)]
    pub image_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<resource_settings::Size>,
    #[serde(skip_serializing)]
    pub cores: Option<i32>,
    #[serde(rename = "referenceVm")]
    pub reference_vm: ReferenceVm,
}
pub mod resource_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Size {
        Basic,
        Standard,
        Performance,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSettingsFragment {
    #[serde(rename = "galleryImageResourceId", default, skip_serializing_if = "Option::is_none")]
    pub gallery_image_resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<resource_settings_fragment::Size>,
    #[serde(rename = "referenceVm", default, skip_serializing_if = "Option::is_none")]
    pub reference_vm: Option<ReferenceVmFragment>,
}
pub mod resource_settings_fragment {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Size {
        Basic,
        Standard,
        Performance,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationEnvironment {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Environment>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationEnvironmentSetting {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EnvironmentSetting>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationGalleryImage {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GalleryImage>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationLab {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Lab>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationLabAccount {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LabAccount>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationUser {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<User>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SizeAvailability {
    #[serde(rename = "sizeCategory", default, skip_serializing_if = "Option::is_none")]
    pub size_category: Option<size_availability::SizeCategory>,
    #[serde(rename = "isAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
}
pub mod size_availability {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SizeCategory {
        Basic,
        Standard,
        Performance,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SizeConfigurationProperties {
    #[serde(rename = "environmentSizes", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_sizes: Vec<EnvironmentSize>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SizeConfigurationPropertiesFragment {
    #[serde(rename = "environmentSizes", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_sizes: Vec<EnvironmentSizeFragment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SizeInfo {
    #[serde(rename = "computeSize", default, skip_serializing_if = "Option::is_none")]
    pub compute_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(rename = "numberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SizeInfoFragment {
    #[serde(rename = "computeSize", default, skip_serializing_if = "Option::is_none")]
    pub compute_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(rename = "numberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserFragment {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserPropertiesFragment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserProperties {
    #[serde(skip_serializing)]
    pub email: Option<String>,
    #[serde(rename = "familyName", skip_serializing)]
    pub family_name: Option<String>,
    #[serde(rename = "givenName", skip_serializing)]
    pub given_name: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "totalUsage", skip_serializing)]
    pub total_usage: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
    #[serde(rename = "latestOperationResult", default, skip_serializing_if = "Option::is_none")]
    pub latest_operation_result: Option<LatestOperationResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserPropertiesFragment {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineDetails {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "rdpAuthority", skip_serializing)]
    pub rdp_authority: Option<String>,
    #[serde(rename = "sshAuthority", skip_serializing)]
    pub ssh_authority: Option<String>,
    #[serde(rename = "privateIpAddress", skip_serializing)]
    pub private_ip_address: Option<String>,
    #[serde(rename = "userName", skip_serializing)]
    pub user_name: Option<String>,
    #[serde(rename = "lastKnownPowerState", skip_serializing)]
    pub last_known_power_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmStateDetails {
    #[serde(rename = "rdpAuthority", skip_serializing)]
    pub rdp_authority: Option<String>,
    #[serde(rename = "sshAuthority", skip_serializing)]
    pub ssh_authority: Option<String>,
    #[serde(rename = "powerState", skip_serializing)]
    pub power_state: Option<String>,
    #[serde(rename = "lastKnownPowerState", skip_serializing)]
    pub last_known_power_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmStateDetailsFragment {}