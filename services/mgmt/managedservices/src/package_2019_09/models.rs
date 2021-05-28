#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistrationDefinitionProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationDefinitionProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub authorizations: Vec<Authorization>,
    #[serde(rename = "registrationDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub registration_definition_name: Option<String>,
    #[serde(rename = "managedByTenantId")]
    pub managed_by_tenant_id: String,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<registration_definition_properties::ProvisioningState>,
    #[serde(rename = "managedByTenantName", skip_serializing)]
    pub managed_by_tenant_name: Option<String>,
}
pub mod registration_definition_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        NotSpecified,
        Accepted,
        Running,
        Ready,
        Creating,
        Created,
        Deleting,
        Deleted,
        Canceled,
        Failed,
        Succeeded,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationDefinitionList {
    #[serde(skip_serializing)]
    pub value: Vec<RegistrationDefinition>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationAssignment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistrationAssignmentProperties>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationAssignmentProperties {
    #[serde(rename = "registrationDefinitionId")]
    pub registration_definition_id: String,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<registration_assignment_properties::ProvisioningState>,
    #[serde(rename = "registrationDefinition", skip_serializing)]
    pub registration_definition: Option<registration_assignment_properties::RegistrationDefinition>,
}
pub mod registration_assignment_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        NotSpecified,
        Accepted,
        Running,
        Ready,
        Creating,
        Created,
        Deleting,
        Deleted,
        Canceled,
        Failed,
        Succeeded,
        Updating,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RegistrationDefinition {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub properties: Option<registration_definition::Properties>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub plan: Option<Plan>,
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(rename = "type", skip_serializing)]
        pub type_: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
    }
    pub mod registration_definition {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct Properties {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub description: Option<String>,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub authorizations: Vec<Authorization>,
            #[serde(rename = "registrationDefinitionName", default, skip_serializing_if = "Option::is_none")]
            pub registration_definition_name: Option<String>,
            #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
            pub provisioning_state: Option<properties::ProvisioningState>,
            #[serde(rename = "manageeTenantId", default, skip_serializing_if = "Option::is_none")]
            pub managee_tenant_id: Option<String>,
            #[serde(rename = "manageeTenantName", default, skip_serializing_if = "Option::is_none")]
            pub managee_tenant_name: Option<String>,
            #[serde(rename = "managedByTenantId", default, skip_serializing_if = "Option::is_none")]
            pub managed_by_tenant_id: Option<String>,
            #[serde(rename = "managedByTenantName", default, skip_serializing_if = "Option::is_none")]
            pub managed_by_tenant_name: Option<String>,
        }
        pub mod properties {
            use super::*;
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            pub enum ProvisioningState {
                NotSpecified,
                Accepted,
                Running,
                Ready,
                Creating,
                Created,
                Deleting,
                Deleted,
                Canceled,
                Failed,
                Succeeded,
                Updating,
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationAssignmentList {
    #[serde(skip_serializing)]
    pub value: Vec<RegistrationAssignment>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceRegistrationDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MarketplaceRegistrationDefinitionProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceRegistrationDefinitionProperties {
    #[serde(rename = "managedByTenantId")]
    pub managed_by_tenant_id: String,
    pub authorizations: Vec<Authorization>,
    #[serde(rename = "offerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub offer_display_name: Option<String>,
    #[serde(rename = "publisherDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_display_name: Option<String>,
    #[serde(rename = "planDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub plan_display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceRegistrationDefinitionList {
    #[serde(skip_serializing)]
    pub value: Vec<MarketplaceRegistrationDefinition>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    pub publisher: String,
    pub product: String,
    pub version: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationList {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Authorization {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "principalIdDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub principal_id_display_name: Option<String>,
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[serde(rename = "delegatedRoleDefinitionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub delegated_role_definition_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
}