#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Azure Video Analyzer for Media access token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessToken {
    #[doc = "The access token."]
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}
impl AccessToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Video Analyzer for Media account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Azure Video Analyzer for Media account properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountPropertiesForPutRequest>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Account {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
            system_data: None,
        }
    }
}
#[doc = "The parameters used to check the availability of the Video Indexer account name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountCheckNameAvailabilityParameters {
    #[doc = "The VideoIndexer account name."]
    pub name: String,
    #[doc = "The type of resource, Microsoft.VideoIndexer/accounts"]
    #[serde(rename = "type")]
    pub type_: account_check_name_availability_parameters::Type,
}
impl AccountCheckNameAvailabilityParameters {
    pub fn new(name: String, type_: account_check_name_availability_parameters::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod account_check_name_availability_parameters {
    use super::*;
    #[doc = "The type of resource, Microsoft.VideoIndexer/accounts"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.VideoIndexer/accounts")]
        MicrosoftVideoIndexerAccounts,
    }
}
#[doc = "The list operation response, that contains the data pools and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountList {
    #[doc = "List of accounts and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Account>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AccountList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Video Analyzer for Media account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountPatch {
    #[serde(flatten)]
    pub tags: Tags,
    #[doc = "Azure Video Analyzer for Media account properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountPropertiesForPatchRequest>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl AccountPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Video Analyzer for Media account properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountPropertiesForPatchRequest {
    #[doc = "The account's tenant id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The account's data-plane ID"]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The media services details"]
    #[serde(rename = "mediaServices", default, skip_serializing_if = "Option::is_none")]
    pub media_services: Option<MediaServicesForPatchRequest>,
    #[doc = "Gets the status of the account at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<account_properties_for_patch_request::ProvisioningState>,
}
impl AccountPropertiesForPatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod account_properties_for_patch_request {
    use super::*;
    #[doc = "Gets the status of the account at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Accepted,
        Provisioning,
        Deleting,
    }
}
#[doc = "Azure Video Analyzer for Media account properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountPropertiesForPutRequest {
    #[doc = "The account's tenant id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The account's data-plane ID"]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The account's name"]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The media services details"]
    #[serde(rename = "mediaServices", default, skip_serializing_if = "Option::is_none")]
    pub media_services: Option<MediaServicesForPutRequest>,
    #[doc = "Gets the status of the account at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<account_properties_for_put_request::ProvisioningState>,
}
impl AccountPropertiesForPutRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod account_properties_for_put_request {
    use super::*;
    #[doc = "Gets the status of the account at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Accepted,
        Provisioning,
        Deleting,
    }
}
#[doc = "The CheckNameAvailability operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "Gets a boolean value that indicates whether the name is available for you to use. If true, the name is available. If false, the name has already been taken."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Gets the reason that a Video Indexer account name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[doc = "Gets an error message explaining the Reason value in more detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_result {
    use super::*;
    #[doc = "Gets the reason that a Video Indexer account name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        AlreadyExists,
    }
}
#[doc = "An Azure Video Analyzer for Media classic account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassicAccount {
    #[doc = "The account's name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The account's location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Azure Video Analyzer for Media classic account properties"]
    #[serde(rename = "mediaServices", default, skip_serializing_if = "Option::is_none")]
    pub media_services: Option<ClassicAccountMediaServices>,
}
impl ClassicAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Video Analyzer for Media classic account properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassicAccountMediaServices {
    #[doc = "The aad application id"]
    #[serde(rename = "aadApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub aad_application_id: Option<String>,
    #[doc = "The aad tenant id"]
    #[serde(rename = "aadTenantId", default, skip_serializing_if = "Option::is_none")]
    pub aad_tenant_id: Option<String>,
    #[doc = "Represents wether the media services is connected or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    #[doc = "Represents if the media services event grid is connected or not"]
    #[serde(rename = "eventGridProviderRegistered", default, skip_serializing_if = "Option::is_none")]
    pub event_grid_provider_registered: Option<bool>,
    #[doc = "The media services name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource group that the media services belong to"]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "Represents wether the media services streaming endpoint has started"]
    #[serde(rename = "streamingEndpointStarted", default, skip_serializing_if = "Option::is_none")]
    pub streaming_endpoint_started: Option<bool>,
    #[doc = "The media services subscriptionId"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl ClassicAccountMediaServices {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Video Analyzer for Media classic account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassicAccountSlim {
    #[doc = "The account's name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The account's location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl ClassicAccountSlim {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Internal error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Access token generation request's parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateAccessTokenParameters {
    #[doc = "The requested permission"]
    #[serde(rename = "permissionType")]
    pub permission_type: generate_access_token_parameters::PermissionType,
    #[doc = "The requested media type"]
    pub scope: generate_access_token_parameters::Scope,
    #[doc = "The video ID"]
    #[serde(rename = "videoId", default, skip_serializing_if = "Option::is_none")]
    pub video_id: Option<String>,
    #[doc = "The project ID"]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}
impl GenerateAccessTokenParameters {
    pub fn new(permission_type: generate_access_token_parameters::PermissionType, scope: generate_access_token_parameters::Scope) -> Self {
        Self {
            permission_type,
            scope,
            video_id: None,
            project_id: None,
        }
    }
}
pub mod generate_access_token_parameters {
    use super::*;
    #[doc = "The requested permission"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PermissionType {
        Contributor,
        Reader,
    }
    #[doc = "The requested media type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scope {
        Video,
        Account,
        Project,
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned,UserAssigned")]
    SystemAssignedUserAssigned,
}
#[doc = "The media services details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaServicesForPatchRequest {
    #[doc = "The user assigned identity to be used to grant permissions"]
    #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity: Option<String>,
}
impl MediaServicesForPatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The media services details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaServicesForPutRequest {
    #[doc = "The media services resource id"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The user assigned identity to be used to grant permissions"]
    #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity: Option<String>,
}
impl MediaServicesForPutRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation detail payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Indicates the action type."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[doc = "Operation display payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation display payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Resource provider of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Localized friendly name for the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Localized friendly description for the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Available operations of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the Resource Provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of user classic accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserClassicAccountList {
    #[doc = "List of classic account names and their location."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClassicAccountSlim>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl UserClassicAccountList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
