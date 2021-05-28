#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentQuotaLimit {
    #[serde(rename = "quotaInformation", default, skip_serializing_if = "Option::is_none")]
    pub quota_information: Option<CurrentQuotaLimitBase>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestStatusDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentQuotaLimitBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "currentValue", skip_serializing)]
    pub current_value: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<ResourceTypesName>,
    #[serde(rename = "quotaPeriod", skip_serializing)]
    pub quota_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", skip_serializing)]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaLimits {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CurrentQuotaLimitBase>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaLimitsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CurrentQuotaLimit>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateGenericQuotaRequestParameters {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CurrentQuotaLimitBase>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubRequest {
    #[serde(skip_serializing)]
    pub limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(rename = "subRequestId", skip_serializing)]
    pub sub_request_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestOneResourceSubmitResponse {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestOneResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestSubmitResponse {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestProperties>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestSubmitResponse201 {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestStatusDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestStatusDetails {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestDetails {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestProperties>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestDetailsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<QuotaRequestDetails>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(rename = "requestSubmitTime", skip_serializing)]
    pub request_submit_time: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubRequest>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestOneResourceProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(rename = "requestSubmitTime", skip_serializing)]
    pub request_submit_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CurrentQuotaLimitBase>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QuotaRequestState {
    Accepted,
    Invalid,
    Succeeded,
    Failed,
    InProgress,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResourceTypesName {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "dedicated")]
    Dedicated,
    #[serde(rename = "lowPriority")]
    LowPriority,
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "serviceSpecific")]
    ServiceSpecific,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoQuotaIncreaseDetail {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutoQuotaIncreaseSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoQuotaIncreaseSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<AqiSettings>,
    #[serde(rename = "onFailure", default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Actions>,
    #[serde(rename = "onSuccess", default, skip_serializing_if = "Option::is_none")]
    pub on_success: Option<Actions>,
    #[serde(rename = "supportTicketAction", default, skip_serializing_if = "Option::is_none")]
    pub support_ticket_action: Option<SupportRequestAction>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AqiSettings {
    #[serde(rename = "autoQuotaIncreaseState", default, skip_serializing_if = "Option::is_none")]
    pub auto_quota_increase_state: Option<AqiState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SupportContactTypes {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "chat")]
    Chat,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportRequestAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<SeverityTypes>,
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "primaryEmailAddress", default, skip_serializing_if = "Option::is_none")]
    pub primary_email_address: Option<String>,
    #[serde(rename = "supportLanguage", default, skip_serializing_if = "Option::is_none")]
    pub support_language: Option<String>,
    #[serde(rename = "preferredContactMethod", default, skip_serializing_if = "Option::is_none")]
    pub preferred_contact_method: Option<ContactMethod>,
    #[serde(rename = "alternateEmailAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub alternate_email_addresses: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SeverityTypes {
    Critical,
    Moderate,
    Minimal,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContactMethod {
    Email,
    Phone,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AqiState {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhoneAction {
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "preferredChannel", default, skip_serializing_if = "Option::is_none")]
    pub preferred_channel: Option<ContactMethod>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailAction {
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailActions {
    #[serde(rename = "emailAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub email_addresses: Vec<EmailAction>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Actions {
    #[serde(rename = "emailActions", default, skip_serializing_if = "Option::is_none")]
    pub email_actions: Option<EmailActions>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExceptionResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ServiceError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ServiceErrorDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceErrorDetail {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservationStatusCode {
    None,
    Pending,
    Active,
    PurchaseError,
    PaymentInstrumentError,
    Split,
    Merged,
    Expired,
    Succeeded,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ErrorResponseCode {
    NotSpecified,
    InternalServerError,
    ServerTimeout,
    AuthorizationFailed,
    BadRequest,
    ClientCertificateThumbprintNotSet,
    InvalidRequestContent,
    OperationFailed,
    HttpMethodNotSupported,
    InvalidRequestUri,
    MissingTenantId,
    InvalidTenantId,
    InvalidReservationOrderId,
    InvalidReservationId,
    ReservationIdNotInReservationOrder,
    ReservationOrderNotFound,
    InvalidSubscriptionId,
    InvalidAccessToken,
    InvalidLocationId,
    UnauthenticatedRequestsThrottled,
    InvalidHealthCheckType,
    Forbidden,
    BillingScopeIdCannotBeChanged,
    AppliedScopesNotAssociatedWithCommerceAccount,
    PatchValuesSameAsExisting,
    RoleAssignmentCreationFailed,
    ReservationOrderCreationFailed,
    ReservationOrderNotEnabled,
    CapacityUpdateScopesFailed,
    UnsupportedReservationTerm,
    ReservationOrderIdAlreadyExists,
    RiskCheckFailed,
    CreateQuoteFailed,
    ActivateQuoteFailed,
    NonsupportedAccountId,
    PaymentInstrumentNotFound,
    MissingAppliedScopesForSingle,
    NoValidReservationsToReRate,
    #[serde(rename = "ReRateOnlyAllowedForEA")]
    ReRateOnlyAllowedForEa,
    OperationCannotBePerformedInCurrentState,
    InvalidSingleAppliedScopesCount,
    InvalidFulfillmentRequestParameters,
    NotSupportedCountry,
    InvalidRefundQuantity,
    PurchaseError,
    BillingCustomerInputError,
    BillingPaymentInstrumentSoftError,
    BillingPaymentInstrumentHardError,
    BillingTransientError,
    BillingError,
    FulfillmentConfigurationError,
    FulfillmentOutOfStockError,
    FulfillmentTransientError,
    FulfillmentError,
    CalculatePriceFailed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "billingPlans", default, skip_serializing_if = "Option::is_none")]
    pub billing_plans: Option<serde_json::Value>,
    #[serde(skip_serializing)]
    pub terms: Vec<ReservationTerm>,
    #[serde(skip_serializing)]
    pub locations: Vec<String>,
    #[serde(rename = "skuProperties", skip_serializing)]
    pub sku_properties: Vec<SkuProperty>,
    #[serde(skip_serializing)]
    pub restrictions: Vec<SkuRestriction>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuRestriction {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationOrderResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i64>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationOrderProperties>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservationBillingPlan {
    Upfront,
    Monthly,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservationTerm {
    #[serde(rename = "P1Y")]
    P1y,
    #[serde(rename = "P3Y")]
    P3y,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PaymentStatus {
    Succeeded,
    Failed,
    Scheduled,
    Cancelled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentDetail {
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "paymentDate", default, skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<String>,
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<Price>,
    #[serde(rename = "billingAccount", default, skip_serializing_if = "Option::is_none")]
    pub billing_account: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentStatus>,
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationOrderBillingPlanInformation {
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "nextPaymentDueDate", default, skip_serializing_if = "Option::is_none")]
    pub next_payment_due_date: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<PaymentDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationOrderProperties {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "requestDateTime", default, skip_serializing_if = "Option::is_none")]
    pub request_date_time: Option<String>,
    #[serde(rename = "createdDateTime", default, skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(rename = "originalQuantity", default, skip_serializing_if = "Option::is_none")]
    pub original_quantity: Option<ReservationQuantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[serde(rename = "planInformation", default, skip_serializing_if = "Option::is_none")]
    pub plan_information: Option<ReservationOrderBillingPlanInformation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reservations: Vec<ReservationResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationResponse {
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i64>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationProperties>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenewPropertiesResponse {
    #[serde(rename = "purchaseProperties", default, skip_serializing_if = "Option::is_none")]
    pub purchase_properties: Option<PurchaseRequest>,
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<renew_properties_response::PricingCurrencyTotal>,
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<renew_properties_response::BillingCurrencyTotal>,
}
pub mod renew_properties_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PricingCurrencyTotal {
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct BillingCurrencyTotal {
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalculatePriceResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CalculatePriceResponseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalculatePriceResponseProperties {
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<calculate_price_response_properties::BillingCurrencyTotal>,
    #[serde(rename = "isBillingPartnerManaged", default, skip_serializing_if = "Option::is_none")]
    pub is_billing_partner_managed: Option<bool>,
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[serde(rename = "skuTitle", default, skip_serializing_if = "Option::is_none")]
    pub sku_title: Option<String>,
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<calculate_price_response_properties::PricingCurrencyTotal>,
    #[serde(rename = "paymentSchedule", default, skip_serializing_if = "Vec::is_empty")]
    pub payment_schedule: Vec<PaymentDetail>,
}
pub mod calculate_price_response_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct BillingCurrencyTotal {
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PricingCurrencyTotal {
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationProperties {
    #[serde(rename = "reservedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<ReservedResourceType>,
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<ReservationQuantity>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "effectiveDateTime", default, skip_serializing_if = "Option::is_none")]
    pub effective_date_time: Option<String>,
    #[serde(rename = "lastUpdatedDateTime", skip_serializing)]
    pub last_updated_date_time: Option<String>,
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[serde(rename = "splitProperties", default, skip_serializing_if = "Option::is_none")]
    pub split_properties: Option<ReservationSplitProperties>,
    #[serde(rename = "mergeProperties", default, skip_serializing_if = "Option::is_none")]
    pub merge_properties: Option<ReservationMergeProperties>,
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[serde(rename = "renewSource", default, skip_serializing_if = "Option::is_none")]
    pub renew_source: Option<String>,
    #[serde(rename = "renewDestination", default, skip_serializing_if = "Option::is_none")]
    pub renew_destination: Option<String>,
    #[serde(rename = "renewProperties", default, skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<RenewPropertiesResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationSplitProperties {
    #[serde(rename = "splitDestinations", default, skip_serializing_if = "Vec::is_empty")]
    pub split_destinations: Vec<String>,
    #[serde(rename = "splitSource", default, skip_serializing_if = "Option::is_none")]
    pub split_source: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationMergeProperties {
    #[serde(rename = "mergeDestination", default, skip_serializing_if = "Option::is_none")]
    pub merge_destination: Option<String>,
    #[serde(rename = "mergeSources", default, skip_serializing_if = "Vec::is_empty")]
    pub merge_sources: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurchaseRequestProperties {
    #[serde(rename = "reservedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<ReservedResourceType>,
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<ReservationQuantity>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[serde(rename = "reservedResourceProperties", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_properties: Option<purchase_request_properties::ReservedResourceProperties>,
}
pub mod purchase_request_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ReservedResourceProperties {
        #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
        pub instance_flexibility: Option<InstanceFlexibility>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchProperties {
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[serde(rename = "renewProperties", default, skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<patch_properties::RenewProperties>,
}
pub mod patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RenewProperties {
        #[serde(rename = "purchaseProperties", default, skip_serializing_if = "Option::is_none")]
        pub purchase_properties: Option<PurchaseRequest>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplitProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub quantities: Vec<i64>,
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MergeProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MergeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MergeProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurchaseRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PurchaseRequestProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PatchProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplitRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SplitProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ExtendedErrorInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedErrorInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorResponseCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedStatusInfo {
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<ReservationStatusCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationOrderList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationOrderResponse>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationResponse>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppliedReservations {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppliedReservationsProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppliedReservationsProperties {
    #[serde(rename = "reservationOrderIds", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_ids: Option<AppliedReservationList>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppliedReservationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResponse>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
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
pub enum InstanceFlexibility {
    On,
    Off,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppliedScopeType {
    Single,
    Shared,
}
pub type AppliedScopes = Vec<String>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingScopeId {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Renew {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationQuantity {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Properties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionScopeProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionScopeProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<ScopeProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservedResourceType {
    VirtualMachines,
    SqlDatabases,
    SuseLinux,
    CosmosDb,
    RedHat,
    SqlDataWarehouse,
    VMwareCloudSimple,
    RedHatOsa,
    Databricks,
    AppService,
    ManagedDisk,
    BlockBlob,
    RedisCache,
    AzureDataExplorer,
    MySql,
    MariaDb,
    PostgreSql,
    DedicatedHost,
    SapHana,
    SqlAzureHybridBenefit,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Price {
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableScopeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableScopeRequestProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableScopeRequestProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}